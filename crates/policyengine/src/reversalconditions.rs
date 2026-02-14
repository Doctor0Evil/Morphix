use crate::aln_core::{
    CapabilityState,
    CapabilityTransitionRequest,
    Decision,
    DecisionReason,
    PolicyStack,
};
use crate::aln_roles::{RoleSet};
use crate::roh_model::RoHScore;
use crate::envelope::EnvelopeContextView;

/// Minimal context, kept pure and immutable.
#[derive(Debug, Clone)]
pub struct ReversalContext<'a> {
    pub base: &'a CapabilityTransitionRequest,
    pub cap_before: CapabilityState,
    pub cap_after: CapabilityState,
    pub roh_before: RoHScore,
    pub roh_after: RoHScore,
    pub roles: &'a RoleSet,
    pub policy_stack: &'a PolicyStack,
    pub envelope_ctx: &'a EnvelopeContextView,
    /// Diagnostic-only: true if this evaluation was triggered from a pure observer path.
    pub diag_event: bool,
}

/// Extension of DecisionReason tailored to irreversible evolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReversalDecision {
    Allowed,
    Denied(DecisionReason),
}

pub fn evaluate_reversal(ctx: &ReversalContext) -> ReversalDecision {
    // 1. Diagnostic isolation: observers can never change capability.
    if ctx.diag_event {
        return ReversalDecision::Denied(DecisionReason::DeniedIllegalDowngradeByNonRegulator);
    }

    // 2. RoH invariants for CapControlledHuman (monotone + ceiling 0.30 already enforced upstream).
    if matches!(ctx.cap_before, CapabilityState::CapControlledHuman) {
        if ctx.roh_after.value > ctx.roh_before.value {
            return ReversalDecision::Denied(DecisionReason::DeniedRoHViolation);
        }
        if ctx.roh_after.value > 0.30 {
            return ReversalDecision::Denied(DecisionReason::DeniedRoHViolation);
        }
    }

    // 3. If this is *not* a neuromorph evolution downgrade, we do not interfere.
    if !is_neuromorph_downgrade(ctx.cap_before, ctx.cap_after) {
        return ReversalDecision::Allowed;
    }

    // 4. Hard constitutional short‑circuit: neuromorph downgrade is structurally forbidden.
    ReversalDecision::Denied(DecisionReason::DeniedIllegalDowngradeByNonRegulator)
}

/// Neuromorph downgrade = any move down the live neuromorph lattice.
fn is_neuromorph_downgrade(from: CapabilityState, to: CapabilityState) -> bool {
    use CapabilityState::*;
    matches!(
        (from, to),
        (CapControlledHuman, CapLabBench)
            | (CapControlledHuman, CapModelOnly)
            | (CapGeneralUse, CapControlledHuman)
            | (CapGeneralUse, CapLabBench)
            | (CapGeneralUse, CapModelOnly)
    )
}
