//! Morphix Guard: fairness–safety labelling observer
//!
//! Role:
//! - Pure, non-actuating diagnostics over CapabilityState, RoH,
//!   BiophysicalEnvelopeSnapshot, TreeOfLifeView, and MicroSociety predicates.
//! - Computes 1D–5D fairness/safety labels with explicit provenance.
//! - Emits only structured diagnostics for logging into .evolve.jsonl and
//!   .donutloop.aln via the existing sovereignty logging layer.
//! - Never mutates CapabilityState, consent, envelopes, ALN shards, or devices.
//!
//! This module is intended for integration as a Pattern I, read-only observer
//! (Tree-of-Life / Neuroprint! style) in the NewRow-Print! / OrganicCPU stack. [file:14][file:10]

use serde::{Deserialize, Serialize};

/// Capability tiers mirrored from NewRowPrint.PolicyEngine / CapabilityState lattice. [file:17]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CapabilityState {
    ModelOnly,
    LabBench,
    ControlledHuman,
    GeneralUse,
}

/// MicroSociety predicates: CALM_STABLE, UNFAIR_DRAIN, etc., as computed by
/// upstream NATURE / metabolic-doctrine layers from TREE and envelope histories. [file:10]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MicroSocietyPredicate {
    CalmStable,
    Overloaded,
    UnfairDrain,
    Recovery,
    BoundarySkimming,
    Other(&'static str),
}

/// Risk-of-Harm score scalar, already governed by .rohmodel.aln
/// (monotone, RoH_after >= RoH_before, RoH <= 0.30 in CapControlledHuman). [file:17]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RoH {
    pub value: f32,
}

/// BiophysicalEnvelopeSnapshot: flattened, read-only snapshot derived from
/// BiophysicalEnvelopeSpec axes at a single epoch. [file:14][file:17]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiophysicalEnvelopeSnapshot {
    // Normalized envelope fractions 0.0–1.0 for key axes.
    // These are projections of the ALN EnvelopeAxis states; no device access. [file:17]
    pub eeg_alpha_frac: f32,
    pub eeg_gamma_frac: f32,
    pub eda_tonic_frac: f32,
    pub bpm_frac: f32,
    pub cognitive_load_warn_frac: f32,
    pub sleep_arousal_warn_frac: f32,
    pub inflammation_warn_frac: f32,
}

/// TreeOfLifeView: the 14–15 TREE asset scalars defined in Tree-of-Life.rs. [file:14]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeOfLifeView {
    pub blood: f32,
    pub oxygen: f32,
    pub wave: f32,
    pub h2o: f32,
    pub time: f32,
    pub decay: f32,
    pub lifeforce: f32,
    pub brain: f32,
    pub smart: f32,
    pub evolve: f32,
    pub power: f32,
    pub tech: f32,
    pub fear: f32,
    pub pain: f32,
    pub nano: f32,
}

/// Minimal MicroSociety view: predicates already computed upstream from
/// multi-subject / multi-role histories (e.g., CALM_STABLE, UNFAIR_DRAIN). [file:10]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MicroSocietyView {
    pub predicates: Vec<MicroSocietyPredicate>,
}

/// Input snapshot for MorphixGuard: single, per-epoch diagnostic view. [file:14][file:10]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphixGuardInput {
    pub capability_state: CapabilityState,
    pub roh: RoH,
    pub envelope: BiophysicalEnvelopeSnapshot,
    pub tree_of_life: TreeOfLifeView,
    pub micro_society: MicroSocietyView,

    // Optional provenance indices to line up with .evolve.jsonl / .donutloop.aln. [file:14][file:17]
    pub evolve_index: Option<u64>,
    pub epoch_index: Option<u64>,
}

/// 1D–5D fairness–safety label primitives.
/// These are purely diagnostic categories; they carry no policy semantics. [file:10]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GuardDimension {
    D1,
    D2,
    D3,
    D4,
    D5,
}

/// Core label enumeration, structured for explicit provenance. [file:10]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MorphixLabel {
    /// 1D scalar: overall fair / within norms.
    D1Fair,
    /// 1D scalar: overall unfair / draining.
    D1UnfairDrainRisk,

    /// 3D composite views over DECAY, LIFEFORCE, and POWER. [file:10]
    D3Fair, // e.g., balanced, low DECAY, adequate LIFEFORCE, non-exploitative POWER.
    D3UnfairDrainRisk,
    D3OverloadRisk,

    /// 5D views DECAY, LIFEFORCE, POWER, FEAR, PAIN / NATURE tokens. [file:10]
    D5BoundarySkimming,
    D5CalmStable,
    D5UnfairDrainConfirmed,
    D5OverloadedRecoveryWindow,
}

/// Provenance descriptor linking a label back to its source fields and shards. [file:14][file:10]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelProvenance {
    /// Dimensions used (1D–5D).
    pub dimension: GuardDimension,
    /// Human-readable explanation surface.
    pub explanation: String,
    /// Source assets / signals that contributed to the label.
    pub sources: Vec<String>,
    /// Shard references (ALN / spec names) that define the semantics. [file:14][file:17]
    pub shard_refs: Vec<String>,
}

/// A single diagnostic label + provenance bundle. Purely advisory. [file:10]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphixDiagnostic {
    pub label: MorphixLabel,
    pub provenance: LabelProvenance,
}

/// Aggregate diagnostics for one epoch / snapshot, ready for logging. [file:14]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphixGuardView {
    /// Echo of input provenance to align with .evolve.jsonl / .donutloop.aln. [file:14][file:17]
    pub capability_state: CapabilityState,
    pub roh_value: f32,
    pub evolve_index: Option<u64>,
    pub epoch_index: Option<u64>,

    /// Primary fairness–safety diagnostics.
    pub diagnostics: Vec<MorphixDiagnostic>,
}

/// Configuration: thresholds for advisory labels only.
/// Loaded and owned by the sovereignty core; MorphixGuard only reads it. [file:14][file:10]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphixGuardConfig {
    /// DECAY threshold beyond which we consider boundary skimming. [file:10]
    pub decay_boundary_thresh: f32,
    /// LIFEFORCE floor for fair / calm-stable states. [file:10]
    pub lifeforce_fair_floor: f32,
    /// POWER fraction above which we consider potential unfair drain. [file:10]
    pub power_unfair_thresh: f32,
    /// FEAR / PAIN thresholds for overload risk. [file:10]
    pub fear_overload_thresh: f32,
    pub pain_overload_thresh: f32,
}

impl MorphixGuardConfig {
    /// Conservative defaults; concrete values should be documented in
    /// morphix_guard.aln and aligned with existing envelope / RoH shards. [file:14][file:17]
    pub fn default() -> Self {
        Self {
            decay_boundary_thresh: 0.70,
            lifeforce_fair_floor: 0.50,
            power_unfair_thresh: 0.70,
            fear_overload_thresh: 0.60,
            pain_overload_thresh: 0.60,
        }
    }
}

/// MorphixGuard: namespace struct with pure, associated functions only.
/// No internal state, no actuation, no IO, no kernel calls. [file:14][file:10]
pub struct MorphixGuard;

impl MorphixGuard {
    /// Core entry: compute pure fairness–safety diagnostics for a single snapshot. [file:10]
    ///
    /// This function:
    /// - Reads governed inputs only (CapabilityState, RoH, envelopes, TreeOfLife, MicroSociety).
    /// - Computes 1D–5D labels based on simple, documented rules.
    /// - Returns a MorphixGuardView suitable for serialization into .evolve.jsonl /
    ///   .donutloop.aln by an external logging layer. [file:14][file:17]
    ///
    /// It MUST NOT:
    /// - Write files, open sockets, call driver APIs.
    /// - Mutate CapabilityState, consent, envelopes, or ALN shards.
    /// - Trigger CapabilityTransitionRequest, ReversalConditions, or PolicyStack paths. [file:10]
    pub fn evaluate(input: &MorphixGuardInput, cfg: &MorphixGuardConfig) -> MorphixGuardView {
        let mut diagnostics = Vec::new();

        let t = &input.tree_of_life;
        let roh = input.roh.value.clamp(0.0, 1.0);

        // 1D fairness view: simple scalar check using LIFEFORCE vs DECAY & RoH. [file:10]
        if t.lifeforce >= cfg.lifeforce_fair_floor && t.decay < cfg.decay_boundary_thresh {
            diagnostics.push(MorphixDiagnostic {
                label: MorphixLabel::D1Fair,
                provenance: LabelProvenance {
                    dimension: GuardDimension::D1,
                    explanation: "Overall fair: lifeforce above floor and decay below boundary threshold."
                        .to_string(),
                    sources: vec![
                        "TreeOfLifeView.lifeforce".into(),
                        "TreeOfLifeView.decay".into(),
                        "RoH.value".into(),
                    ],
                    shard_refs: vec![
                        "Tree-of-Life.md/TREE-LIFEFORCE".into(),
                        "Tree-of-Life.md/TREE-DECAY".into(),
                        ".rohmodel.aln".into(),
                    ],
                },
            });
        } else {
            diagnostics.push(MorphixDiagnostic {
                label: MorphixLabel::D1UnfairDrainRisk,
                provenance: LabelProvenance {
                    dimension: GuardDimension::D1,
                    explanation: "Elevated unfair-drain risk: lifeforce depleted or decay near boundary."
                        .to_string(),
                    sources: vec![
                        "TreeOfLifeView.lifeforce".into(),
                        "TreeOfLifeView.decay".into(),
                        "RoH.value".into(),
                    ],
                    shard_refs: vec![
                        "Tree-of-Life.md/TREE-LIFEFORCE".into(),
                        "Tree-of-Life.md/TREE-DECAY".into(),
                        ".rohmodel.aln".into(),
                    ],
                },
            });
        }

        // 3D fairness view over DECAY, LIFEFORCE, POWER. [file:10]
        if t.decay < cfg.decay_boundary_thresh
            && t.lifeforce >= cfg.lifeforce_fair_floor
            && t.power < cfg.power_unfair_thresh
        {
            diagnostics.push(MorphixDiagnostic {
                label: MorphixLabel::D3Fair,
                provenance: LabelProvenance {
                    dimension: GuardDimension::D3,
                    explanation: "3D fair energy budget: decay low, lifeforce adequate, power below unfair threshold."
                        .to_string(),
                    sources: vec![
                        "TreeOfLifeView.decay".into(),
                        "TreeOfLifeView.lifeforce".into(),
                        "TreeOfLifeView.power".into(),
                    ],
                    shard_refs: vec![
                        "Tree-of-Life.md/TREE-DECAY".into(),
                        "Tree-of-Life.md/TREE-LIFEFORCE".into(),
                        "Tree-of-Life.md/TREE-POWER".into(),
                    ],
                },
            });
        } else if t.power >= cfg.power_unfair_thresh && t.lifeforce < cfg.lifeforce_fair_floor {
            diagnostics.push(MorphixDiagnostic {
                label: MorphixLabel::D3UnfairDrainRisk,
                provenance: LabelProvenance {
                    dimension: GuardDimension::D3,
                    explanation: "3D unfair-drain risk: power high while lifeforce is depleted under elevated decay."
                        .to_string(),
                    sources: vec![
                        "TreeOfLifeView.decay".into(),
                        "TreeOfLifeView.lifeforce".into(),
                        "TreeOfLifeView.power".into(),
                    ],
                    shard_refs: vec![
                        "Tree-of-Life.md/TREE-DECAY".into(),
                        "Tree-of-Life.md/TREE-LIFEFORCE".into(),
                        "Tree-of-Life.md/TREE-POWER".into(),
                        "MetabolicDoctrine.* (UNFAIR_DRAIN)".into(),
                    ],
                },
            });
        } else if roh >= cfg.decay_boundary_thresh {
            diagnostics.push(MorphixDiagnostic {
                label: MorphixLabel::D3OverloadRisk,
                provenance: LabelProvenance {
                    dimension: GuardDimension::D3,
                    explanation: "3D overload risk: RoH and decay near boundary; consider cooldown in analysis."
                        .to_string(),
                    sources: vec![
                        "TreeOfLifeView.decay".into(),
                        "RoH.value".into(),
                    ],
                    shard_refs: vec![
                        ".rohmodel.aln".into(),
                        "BiophysicalEnvelopeSpec/*-overload".into(),
                    ],
                },
            });
        }

        // 5D view including FEAR and PAIN plus NATURE predicates. [file:10]
        let has_calm = input
            .micro_society
            .predicates
            .contains(&MicroSocietyPredicate::CalmStable);
        let has_unfair = input
            .micro_society
            .predicates
            .contains(&MicroSocietyPredicate::UnfairDrain);
        let has_overload = input
            .micro_society
            .predicates
            .contains(&MicroSocietyPredicate::Overloaded);
        let has_boundary = input
            .micro_society
            .predicates
            .contains(&MicroSocietyPredicate::BoundarySkimming);

        // Calm-stable.
        if has_calm && t.decay < cfg.decay_boundary_thresh && t.fear < cfg.fear_overload_thresh {
            diagnostics.push(MorphixDiagnostic {
                label: MorphixLabel::D5CalmStable,
                provenance: LabelProvenance {
                    dimension: GuardDimension::D5,
                    explanation: "5D calm-stable micro-society: CALM_STABLE predicate, low decay, low fear/pain."
                        .to_string(),
                    sources: vec![
                        "MicroSociety.CALM_STABLE".into(),
                        "TreeOfLifeView.decay".into(),
                        "TreeOfLifeView.fear".into(),
                        "TreeOfLifeView.pain".into(),
                    ],
                    shard_refs: vec![
                        "MetabolicDoctrine.NATURE/CALM_STABLE".into(),
                        "Tree-of-Life.md/TREE-DECAY".into(),
                        "Tree-of-Life.md/TREE-FEAR".into(),
                        "Tree-of-Life.md/TREE-PAIN".into(),
                    ],
                },
            });
        }

        // Boundary-skimming: high DECAY but not yet overloaded, with BOUNDARY_SKIMMING. [file:10]
        if has_boundary && t.decay >= cfg.decay_boundary_thresh && roh < 0.30 {
            diagnostics.push(MorphixDiagnostic {
                label: MorphixLabel::D5BoundarySkimming,
                provenance: LabelProvenance {
                    dimension: GuardDimension::D5,
                    explanation: "5D boundary skimming: boundary-skimming predicate and decay near RoH ceiling."
                        .to_string(),
                    sources: vec![
                        "MicroSociety.BOUNDARY_SKIMMING".into(),
                        "TreeOfLifeView.decay".into(),
                        "RoH.value".into(),
                    ],
                    shard_refs: vec![
                        "MetabolicDoctrine.NATURE/BOUNDARY_SKIMMING".into(),
                        ".rohmodel.aln".into(),
                        "BiophysicalEnvelopeSpec/*-warn".into(),
                    ],
                },
            });
        }

        // Unfair drain confirmed: UNFAIR_DRAIN + lifeforce low + power high. [file:10]
        if has_unfair && t.lifeforce < cfg.lifeforce_fair_floor && t.power >= cfg.power_unfair_thresh {
            diagnostics.push(MorphixDiagnostic {
                label: MorphixLabel::D5UnfairDrainConfirmed,
                provenance: LabelProvenance {
                    dimension: GuardDimension::D5,
                    explanation: "5D unfair drain confirmed: UNFAIR_DRAIN predicate, low lifeforce, high power."
                        .to_string(),
                    sources: vec![
                        "MicroSociety.UNFAIR_DRAIN".into(),
                        "TreeOfLifeView.lifeforce".into(),
                        "TreeOfLifeView.power".into(),
                    ],
                    shard_refs: vec![
                        "MetabolicDoctrine.UnfairDrain".into(),
                        "Tree-of-Life.md/TREE-LIFEFORCE".into(),
                        "Tree-of-Life.md/TREE-POWER".into(),
                    ],
                },
            });
        }

        // Overloaded recovery window: OVERLOADED + high fear/pain but RoH not yet at ceiling. [file:10]
        if has_overload
            && (t.fear >= cfg.fear_overload_thresh || t.pain >= cfg.pain_overload_thresh)
            && roh < 0.30
        {
            diagnostics.push(MorphixDiagnostic {
                label: MorphixLabel::D5OverloadedRecoveryWindow,
                provenance: LabelProvenance {
                    dimension: GuardDimension::D5,
                    explanation: "5D overloaded recovery window: OVERLOADED predicate with high fear/pain under RoH ceiling."
                        .to_string(),
                    sources: vec![
                        "MicroSociety.OVERLOADED".into(),
                        "TreeOfLifeView.fear".into(),
                        "TreeOfLifeView.pain".into(),
                        "RoH.value".into(),
                    ],
                    shard_refs: vec![
                        "MetabolicDoctrine.Overloaded".into(),
                        "Tree-of-Life.md/TREE-FEAR".into(),
                        "Tree-of-Life.md/TREE-PAIN".into(),
                        ".rohmodel.aln".into(),
                    ],
                },
            });
        }

        MorphixGuardView {
            capability_state: input.capability_state,
            roh_value: roh,
            evolve_index: input.evolve_index,
            epoch_index: input.epoch_index,
            diagnostics,
        }
    }
}
