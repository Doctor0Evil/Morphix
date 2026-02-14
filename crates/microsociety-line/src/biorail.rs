//! BioRail Scalar Gate and BioLoad Terrasafe Guard
//!
//! This module formalizes the 1D scalar rail `b ∈ [0,1]` as a monotone
//! projection of the 5D identity (BioState, NeuroState, Lifeforce, Context,
//! Sovereignty), and couples it to territorial bioload ceilings via the
//! BioLoad Terrasafe Guard.[file:4]
//!
//! It is substrate‑agnostic: neural bands, hydrogels, vascular nanoswarms,
//! XR fields, and porous‑stone Jetson‑Line sites all map into the same
//! scalar corridor, with per‑zone corridors and territorial ceilings
//! defined in configuration.[file:3][file:4]

use std::cmp::Ordering;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::state::{SiteId, SiteState, TerritoryScale, WorldSnapshot};
use crate::deeds::{ProposedDeed, DeedEffect};
use crate::ethics::EthicsDecision;

/// Fixed Neuromorph‑GOD / Tree‑of‑Life limits.[file:2][file:4]
pub const ROH_MAX: f64 = 0.3;
pub const DECAY_MAX: f64 = 1.0;

/// Per‑zone corridor for the scalar rail b.[file:3]
#[derive(Clone, Debug)]
pub struct BioRailZone {
    pub id: ZoneTag,
    /// Lower and upper bounds for b in this zone (inclusive).
    pub b_min: f64,
    pub b_max: f64,
}

/// Simple tag for substrate / anatomical context.[file:4]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ZoneTag {
    NeuralBand,
    VascularConduit,
    HydrogelPatch,
    XrField,
    JetsonLineSite,
}

/// Territorial bioload ceilings for the Terrasafe guard.[file:4]
#[derive(Clone, Debug)]
pub struct TerrasafeCeilings {
    pub body_max: f64,
    pub room_max: f64,
    pub grid_max: f64,
}

/// Configuration bundle for the guard.[file:4]
#[derive(Clone, Debug)]
pub struct BioRailConfig {
    pub zones: Vec<BioRailZone>,
    pub terrasafe: TerrasafeCeilings,
    /// POWER ≤ k·CHURCH multiplier.
    pub power_church_k: f64,
}

/// Fast view of the 5D identity used for projection.[file:4]
#[derive(Clone, Debug)]
pub struct Identity5D {
    pub biostate_fatigue: f64,
    pub biostate_inflammation: f64,
    pub neurostate_fear: f64,
    pub neurostate_stimulation: f64,
    pub lifeforce_level: f64,
    pub lifeforce_drain: f64,
    pub roh_slice: f64,
    pub decay: f64,
    pub context_territorial_load: f64,
    pub context_pollution: f64,
    pub sovereignty_trust: f64,
    pub sovereignty_consent: bool,
}

impl Identity5D {
    /// Clamp all inputs into [0,1] where applicable.[file:4]
    pub fn clamped(self) -> Self {
        fn c(x: f64) -> f64 {
            if x.is_nan() { 0.0 } else { x.max(0.0).min(1.0) }
        }
        Self {
            biostate_fatigue: c(self.biostate_fatigue),
            biostate_inflammation: c(self.biostate_inflammation),
            neurostate_fear: c(self.neurostate_fear),
            neurostate_stimulation: c(self.neurostate_stimulation),
            lifeforce_level: c(self.lifeforce_level),
            lifeforce_drain: c(self.lifeforce_drain),
            roh_slice: c(self.roh_slice / ROH_MAX), // normalized to corridor.[file:3]
            decay: c(self.decay / DECAY_MAX),
            context_territorial_load: c(self.context_territorial_load),
            context_pollution: c(self.context_pollution),
            sovereignty_trust: c(self.sovereignty_trust),
            sovereignty_consent: self.sovereignty_consent,
        }
    }
}
