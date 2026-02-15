#![forbid(unsafe_code)]

use std::marker::PhantomData;

use crate::eco_adapter::{EcoContext, ImpactScore};

pub struct Corridor<const ID: u32>;

pub enum NoCoerciveChannels {}
pub enum DynamicConsent {}

mod sealed {
    pub trait Sealed {}
}
use sealed::Sealed;

impl<const ID: u32> Sealed for Corridor<ID> {}
impl Sealed for NoCoerciveChannels {}
impl Sealed for DynamicConsent {}

/// Type-level neurorights + corridor binding.
pub struct CoreEcoEngine<const ID: u32> {
    _corridor: PhantomData<Corridor<ID>>,
    _consent: PhantomData<DynamicConsent>,
    _coercion: PhantomData<NoCoerciveChannels>,
}

impl<const ID: u32> CoreEcoEngine<ID> {
    pub const ENGINE_NAME: &'static str = "core_eco_engine_v1";

    pub const fn new() -> Self {
        Self {
            _corridor: PhantomData,
            _consent: PhantomData,
            _coercion: PhantomData,
        }
    }

    pub fn score(&self, ctx: &EcoContext) -> ImpactScore {
        // Placeholder scoring: corridor-safe, neurorights-safe by construction.
        let base = 0.7_f32;
        ImpactScore::clamped(
            base,
            format!("Corridor {} impact for dataset={} (stub).", ID, ctx.dataset_id),
        )
    }
}
