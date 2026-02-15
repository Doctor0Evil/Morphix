use std::time::SystemTime;

use crate::fpic::{FpicStatus, FpicToken};
use crate::care::{CareAttestable, CareAttestation};

/// Context passed to any neuromorph operation before execution.
pub struct SovereignContext<'a> {
    pub fpic: &'a FpicToken,
    pub care: &'a CareAttestation,
    pub now: SystemTime,
}

/// Object-safe trait: any runtime action must pass this check first.
/// AI-chat and SNC orchestrators call `check_fpic_and_care` before
/// generating or executing neuromorphic code.
pub trait SovereignRuntimeGuard: CareAttestable + Send + Sync {
    fn check_fpic_and_care(&self, ctx: &SovereignContext<'_>) -> Result<(), String> {
        // 1. FPIC veto and freshness.[file:69]
        match ctx.fpic.status(ctx.now) {
            FpicStatus::Granted => {}
            FpicStatus::Denied | FpicStatus::Revoked => {
                return Err("FPIC veto or stale consent: operation forbidden.".into());
            }
        }

        // 2. CARE alignment.
        if !ctx.care.is_fully_care_aligned() {
            return Err("CARE alignment missing or incomplete.".into());
        }

        Ok(())
    }
}
