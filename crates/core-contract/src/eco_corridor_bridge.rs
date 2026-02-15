use crate::eco_adapter::{EcoContext, EcoImpactAdapter, ImpactScore};
use crate::eco_core_engine::CoreEcoEngine;

/// Const-generic, corridor-bound engine wrapped as a dynamic adapter.
pub struct CorridorBoundScoreEngine<const ID: u32> {
    engine: CoreEcoEngine<ID>,
}

impl<const ID: u32> CorridorBoundScoreEngine<ID> {
    pub const fn new() -> Self {
        Self {
            engine: CoreEcoEngine::<ID>::new(),
        }
    }
}

impl<const ID: u32> EcoImpactAdapter for CorridorBoundScoreEngine<ID> {
    fn name(&self) -> &'static str {
        CoreEcoEngine::<ID>::ENGINE_NAME
    }

    fn compute_impact(&self, ctx: &EcoContext) -> ImpactScore {
        self.engine.score(ctx)
    }
}
