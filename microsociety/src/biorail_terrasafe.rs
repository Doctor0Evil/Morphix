//! BioRailScalarGate + BioLoadTerrasafeGuard
//!
//! Nonfictional enforcement spine for the Jetson-Line / MicroSociety stack:
//! - Monotone scalar rail b_i ∈ [0,1] as a projection of 5D identity
//!   (BioState, NeuroState, Lifeforce, Context, Sovereignty).
//! - Hard biophysical ceilings: RoH ≤ 0.3, DECAY ≤ 1.0, Lifeforce floors,
//!   computebioload ceilings at body/room/grid, POWER ≤ k·CHURCH.
//! - Justice metrics (HPCC, ERG, TECR) act only as corridor tuners
//!   (tighten limits, force repair modes), never as direct actuators.
//! - Diagnostics (including BEAST/PLAGUE) are ROLE=DIAGNOSTIC-ONLY,
//!   NOACTUATION: only scalar guards decide actuation.
//!
//! This file is designed to plug into an existing Jetson-Line crate where
//! Site, World, Deed, and justice/diagnostic types are already defined. [file:4][file:2]

use std::cmp::Ordering;

/// Bounded scalar in [0,1] used for rails and normalized views. [file:4]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RailScalar(f64);

impl RailScalar {
    pub fn new_clamped(x: f64) -> Self {
        let v = if x.is_nan() { 0.0 } else if x < 0.0 { 0.0 } else if x > 1.0 { 1.0 } else { x };
        RailScalar(v)
    }

    pub fn value(self) -> f64 {
        self.0
    }

    pub fn max(self, other: RailScalar) -> RailScalar {
        RailScalar::new_clamped(self.0.max(other.0))
    }

    pub fn min(self, other: RailScalar) -> RailScalar {
        RailScalar::new_clamped(self.0.min(other.0))
    }

    pub fn cmp(&self, other: &RailScalar) -> Ordering {
        self.0.partial_cmp(&other.0).unwrap_or(Ordering::Equal)
    }
}

/// Core biophysical envelopes needed for the rail. [file:4][file:2]
#[derive(Debug, Clone)]
pub struct BioEnvelope {
    pub roh: f64,          // Risk-of-Harm slice, must be ≤ 0.3. [file:2][file:4]
    pub decay: f64,        // DECAY, normalized, must be ≤ 1.0. [file:2]
    pub lifeforce: f64,    // Lifeforce scalar, must remain within [lf_min, lf_max]. [file:2]
    pub lifeforce_min: f64,
    pub lifeforce_max: f64,
}

/// 5D identity components already present in your stack. [file:4][file:3]
#[derive(Debug, Clone)]
pub struct FiveDIdentity {
    pub biostate_load: f64,     // fatigue, inflammation, metabolic overhead (normalized). [file:4]
    pub neurostate_fear: f64,   // FEAR envelope slice (normalized). [file:3]
    pub lifeforce: f64,         // same as in BioEnvelope, kept for convenience.
    pub context_load: f64,      // territorial eco-impact, XR intensity, exposure (normalized). [file:4][file:2]
    pub sovereignty_trust: f64, // trust/consent in [0,1]. [file:4][file:2]
}

/// Territorial bioload views from computebioload. [file:4][file:2]
#[derive(Debug, Clone)]
pub struct BioLoadView {
    pub body: RailScalar,
    pub room: RailScalar,
    pub grid: RailScalar,
    pub body_max: RailScalar,
    pub room_max: RailScalar,
    pub grid_max: RailScalar,
}

/// POWER/CHURCH slice per site. [file:3][file:2]
#[derive(Debug, Clone)]
pub struct PowerChurchState {
    pub power: f64,
    pub church: f64,
    /// Global k: POWER ≤ k·CHURCH. [file:2][file:3]
    pub k_ratio: f64,
}

/// Justice metrics snapshot (diagnostic only). [file:3][file:2]
#[derive(Debug, Clone)]
pub struct JusticeMetrics {
    pub hpcc: f64, // Habit-Pollution Coupling Coefficient. [file:3]
    pub erg: f64,  // Exposure-Responsibility Gap. [file:3]
    pub tecr: f64, // Token-Enforced Collapse Rate. [file:3]
}

/// Justice corridor configuration; only tightens/relaxes scalar ceilings. [file:3][file:2]
#[derive(Debug, Clone)]
pub struct JusticeCorridorConfig {
    pub hpcc_max: f64,
    pub erg_max: f64,
    pub tecr_max: f64,
    /// Multipliers applied when justice corridors are stressed; they only shrink ceilings. [file:3]
    pub tightening_factor: f64, // e.g., 0.8
}

/// Flags for diagnostics like BEAST/PLAGUE, strictly non-actuating. [file:6][file:2]
#[derive(Debug, Clone)]
pub struct DiagnosticFlags {
    pub beast_tag: bool,
    pub plague_tag: bool,
    pub unfair_drain: bool,
    pub role_diagnostic_only: bool, // must be true structurally. [file:6][file:2]
}

/// Site-local view needed for the BioRail computation and gating. [file:4][file:3]
#[derive(Debug, Clone)]
pub struct SiteView {
    pub id: usize,
    pub bio_env: BioEnvelope,
    pub identity_5d: FiveDIdentity,
    pub bioload_view: BioLoadView,
    pub power_church: PowerChurchState,
    pub justice_metrics: JusticeMetrics,
    pub justice_cfg: JusticeCorridorConfig,
    pub diag: DiagnosticFlags,
}

/// Corridor configuration for the biosignature rail. [file:4][file:3]
#[derive(Debug, Clone)]
pub struct BioRailConfig {
    /// Allowed corridor for b_i at this site/zone. [file:4][file:3]
    pub corridor_min: RailScalar,
    pub corridor_max: RailScalar,
}

/// Verdict from BioRail/Terrasafe gating. [file:4][file:3]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GateVerdict {
    Allow,
    Downscale,
    Block,
    ForceRepair,
}

/// Proposed change summary used for prediction; this is computed upstream from deeds. [file:4][file:3]
#[derive(Debug, Clone)]
pub struct ProposedChange {
    /// Predicted post-change 5D identity deltas (additive). [file:4]
    pub delta_biostate_load: f64,
    pub delta_neurostate_fear: f64,
    pub delta_lifeforce: f64,
    pub delta_context_load: f64,
    pub delta_sovereignty_trust: f64,
    /// Predicted post-change envelopes. [file:4]
    pub delta_roh: f64,
    pub delta_decay: f64,
    pub delta_lifeforce_env: f64,
    /// Predicted bioload deltas at each territory. [file:4]
    pub delta_bioload_body: f64,
    pub delta_bioload_room: f64,
    pub delta_bioload_grid: f64,
    /// Predicted POWER delta. [file:3]
    pub delta_power: f64,
}

/// Main guard that couples the scalar BioRail with BioLoad Terrasafe and POWER/CHURCH caps. [file:4][file:3][file:2]
pub struct BioRailTerrasafeGuard;

impl BioRailTerrasafeGuard {
    /// Compute monotone biosignature rail b_i from 5D identity and envelopes. [file:4][file:3]
    ///
    /// Design:
    /// - Increasing risk components (bioload-like, FEAR-like, RoH, DECAY, context load)
    ///   must not decrease b_i (monotonic safety). [file:4][file:3]
    /// - Increasing sovereignty_trust may reduce b_i within bounds, reflecting improved stewardship. [file:4]
    pub fn compute_biosignature(site: &SiteView) -> RailScalar {
        let id = &site.identity_5d;
        let env = &site.bio_env;

        // Normalize components into [0,1] contributions.
        let roh_norm = (env.roh / 0.3).clamp(0.0, 1.0);        // RoH ≤ 0.3 corridor. [file:2]
        let decay_norm = env.decay.clamp(0.0, 1.0);            // DECAY ≤ 1.0. [file:2]
        let lf_band_width = (env.lifeforce_max - env.lifeforce_min).max(1e-9);
        let lf_pos = ((env.lifeforce - env.lifeforce_min) / lf_band_width).clamp(0.0, 1.0);
        let lf_risk = 1.0 - lf_pos; // more risk toward lower lifeforce. [file:2]

        let bio_load = id.biostate_load.clamp(0.0, 1.0);
        let fear = id.neurostate_fear.clamp(0.0, 1.0);         // FEAR slice. [file:3]
        let ctx = id.context_load.clamp(0.0, 1.0);
        let sovereign = id.sovereignty_trust.clamp(0.0, 1.0);

        // Risk-weighted aggregation; all risk components push b upward. [file:4][file:3]
        let risk_sum =
            0.18 * bio_load +
            0.18 * fear +
            0.18 * ctx +
            0.18 * roh_norm +
            0.18 * decay_norm +
            0.10 * lf_risk;

        // Sovereignty/trust can only reduce risk, never create it. [file:4]
        let sovereign_relief = 0.4 * sovereign;

        let raw = (risk_sum - sovereign_relief).clamp(0.0, 1.0);
        RailScalar::new_clamped(raw)
    }

    /// Apply justice metric tightening to the effective corridors and ceilings. [file:3][file:2]
    fn apply_justice_tuning(site: &SiteView,
                            rail_cfg: &BioRailConfig,
                            bioload: &BioLoadView) -> (BioRailConfig, BioLoadView) {
        let jm = &site.justice_metrics;
        let cfg = &site.justice_cfg;

        // If any justice metric exceeds its corridor, tighten corridors multiplicatively;
        // never loosen beyond the baseline. [file:3]
        let stressed =
            (jm.hpcc > cfg.hpcc_max) ||
            (jm.erg > cfg.erg_max) ||
            (jm.tecr > cfg.tecr_max);

        if !stressed {
            return (rail_cfg.clone(), bioload.clone());
        }

        let factor = cfg.tightening_factor.clamp(0.0, 1.0);

        // Narrow the b_i corridor symmetrically around its midpoint. [file:3]
        let mid = 0.5 * (rail_cfg.corridor_min.value() + rail_cfg.corridor_max.value());
        let half_width = 0.5 * (rail_cfg.corridor_max.value() - rail_cfg.corridor_min.value()) * factor;
        let tuned_min = RailScalar::new_clamped(mid - half_width);
        let tuned_max = RailScalar::new_clamped(mid + half_width);

        // Shrink bioload ceilings; actual views remain unchanged. [file:3]
        let tuned_bioload = BioLoadView {
            body: bioload.body,
            room: bioload.room,
            grid: bioload.grid,
            body_max: RailScalar::new_clamped(bioload.body_max.value() * factor),
            room_max: RailScalar::new_clamped(bioload.room_max.value() * factor),
            grid_max: RailScalar::new_clamped(bioload.grid_max.value() * factor),
        };

        let tuned_cfg = BioRailConfig {
            corridor_min: tuned_min,
            corridor_max: tuned_max,
        };

        (tuned_cfg, tuned_bioload)
    }

    /// Check RoH, DECAY, Lifeforce invariants on the predicted envelopes. [file:2]
    fn check_envelopes(pred_env: &BioEnvelope) -> bool {
        if pred_env.roh > 0.3 + 1e-9 { // RoH ≤ 0.3 hard ceiling. [file:2]
            return false;
        }
        if pred_env.decay > 1.0 + 1e-9 { // DECAY ≤ 1.0. [file:2]
            return false;
        }
        if pred_env.lifeforce < pred_env.lifeforce_min - 1e-9 {
            return false;
        }
        if pred_env.lifeforce > pred_env.lifeforce_max + 1e-9 {
            return false;
        }
        true
    }

    /// Check BioLoad Terrasafe ceilings for predicted values. [file:4][file:2]
    fn check_bioload(pred: &BioLoadView) -> bool {
        pred.body.value() <= pred.body_max.value() + 1e-9 &&
        pred.room.value() <= pred.room_max.value() + 1e-9 &&
        pred.grid.value() <= pred.grid_max.value() + 1e-9
    }

    /// Check POWER ≤ k·CHURCH constraint after the proposed change. [file:3][file:2]
    fn check_power_church(pred_pc: &PowerChurchState) -> bool {
        let k = pred_pc.k_ratio.max(0.0);
        let allowed_power = k * pred_pc.church.max(0.0);
        pred_pc.power <= allowed_power + 1e-9
    }

    /// Compute predicted post-change state slices needed for gating. [file:4][file:3]
    fn predict_post_state(site: &SiteView,
                          change: &ProposedChange,
                          tuned_bioload_max: &BioLoadView)
                          -> (BioEnvelope, BioLoadView, PowerChurchState, FiveDIdentity)
    {
        let mut env = site.bio_env.clone();
        env.roh = (env.roh + change.delta_roh).max(0.0);
        env.decay = (env.decay + change.delta_decay).max(0.0);
        env.lifeforce = env.lifeforce + change.delta_lifeforce_env;

        let mut id = site.identity_5d.clone();
        id.biostate_load = (id.biostate_load + change.delta_biostate_load).clamp(0.0, 1.0);
        id.neurostate_fear = (id.neurostate_fear + change.delta_neurostate_fear).clamp(0.0, 1.0);
        id.lifeforce = id.lifeforce + change.delta_lifeforce;
        id.context_load = (id.context_load + change.delta_context_load).clamp(0.0, 1.0);
        id.sovereignty_trust = (id.sovereignty_trust + change.delta_sovereignty_trust).clamp(0.0, 1.0);

        let mut bl = site.bioload_view.clone();
        bl.body = RailScalar::new_clamped(bl.body.value() + change.delta_bioload_body);
        bl.room = RailScalar::new_clamped(bl.room.value() + change.delta_bioload_room);
        bl.grid = RailScalar::new_clamped(bl.grid.value() + change.delta_bioload_grid);
        // ceilings come from tuned_bioload_max. [file:4][file:3]
        bl.body_max = tuned_bioload_max.body_max;
        bl.room_max = tuned_bioload_max.room_max;
        bl.grid_max = tuned_bioload_max.grid_max;

        let mut pc = site.power_church.clone();
        pc.power = (pc.power + change.delta_power).max(0.0);

        (env, bl, pc, id)
    }

    /// Main guard entrypoint.
    ///
    /// - Computes current and predicted biosignature b_i.
    /// - Applies justice tuning to corridors and bioload ceilings.
    /// - Enforces:
    ///   * RoH ≤ 0.3, DECAY ≤ 1.0, Lifeforce within [min,max]. [file:2]
    ///   * bioload_body/room/grid ≤ max ceilings. [file:4][file:2]
    ///   * POWER ≤ k·CHURCH. [file:3][file:2]
    ///   * b_i_after within tuned corridor.
    /// - Returns GateVerdict used by higher-level deed engine to block/downscale/repair. [file:4][file:3]
    pub fn gate(site: &SiteView,
                base_cfg: &BioRailConfig,
                proposed: &ProposedChange) -> GateVerdict
    {
        // Hard requirement: diagnostics are non-actuating; we ignore them in gating
        // except as evidence later in logs. [file:6][file:2]
        debug_assert!(site.diag.role_diagnostic_only);

        // Justice metrics only tune corridors; get tuned cfg and bioload ceilings. [file:3]
        let (tuned_cfg, tuned_bioload_max) =
            Self::apply_justice_tuning(site, base_cfg, &site.bioload_view);

        // Compute current biosignature (for monotonicity checks if needed). [file:4]
        let current_b = Self::compute_biosignature(site);

        // Predict post-change slices. [file:4]
        let (pred_env, pred_bioload, pred_pc, pred_id) =
            Self::predict_post_state(site, proposed, &tuned_bioload_max);

        // Enforce envelope invariants first. [file:2]
        if !Self::check_envelopes(&pred_env) {
            return GateVerdict::ForceRepair;
        }

        // Enforce BioLoad Terrasafe ceilings. [file:4]
        if !Self::check_bioload(&pred_bioload) {
            return GateVerdict::ForceRepair;
        }

        // Enforce POWER ≤ k·CHURCH caps. [file:3][file:2]
        if !Self::check_power_church(&pred_pc) {
            return GateVerdict::Block;
        }

        // Compute predicted biosignature under new 5D identity. [file:4]
        let pred_site_view = SiteView {
            id: site.id,
            bio_env: pred_env.clone(),
            identity_5d: pred_id,
            bioload_view: pred_bioload.clone(),
            power_church: pred_pc.clone(),
            justice_metrics: site.justice_metrics.clone(),
            justice_cfg: site.justice_cfg.clone(),
            diag: site.diag.clone(),
        };
        let pred_b = Self::compute_biosignature(&pred_site_view);

        // Corridor check on b_i. [file:4][file:3]
        if pred_b.value() < tuned_cfg.corridor_min.value() ||
           pred_b.value() > tuned_cfg.corridor_max.value()
        {
            // If we are leaving the corridor, classify between Downscale vs ForceRepair
            // depending on whether risk is increasing compared to current b. [file:4]
            if pred_b.value() > current_b.value() {
                GateVerdict::ForceRepair
            } else {
                GateVerdict::Downscale
            }
        } else {
            GateVerdict::Allow
        }
    }
}
