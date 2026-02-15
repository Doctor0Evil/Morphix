use std::collections::HashMap;

use crate::eco_adapter::{EcoContext, EcoImpactAdapter, EcoImpactAdapterBox, ImpactScore};

/// Simple in-memory registry of named eco-impact adapters.
/// AI-chat or orchestration layers can select adapters at runtime
/// based on policy, corridor, or SNC configuration.[file:71]
pub struct EcoImpactRegistry {
    adapters: HashMap<String, EcoImpactAdapterBox>,
}

impl EcoImpactRegistry {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }

    pub fn register_adapter<A>(&mut self, adapter: A)
    where
        A: EcoImpactAdapter + 'static,
    {
        let name = adapter.name().to_string();
        self.adapters.insert(name, Box::new(adapter));
    }

    pub fn list_adapters(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }

    /// Core call site used by AI-chat / SNC: pick an adapter by name
    /// and compute an ImpactScore for the given EcoContext.
    pub fn compute_with(
        &self,
        adapter_name: &str,
        ctx: &EcoContext,
    ) -> Result<ImpactScore, String> {
        let adapter = self
            .adapters
            .get(adapter_name)
            .ok_or_else(|| format!("Unknown eco adapter: {adapter_name}"))?;

        Ok(adapter.compute_impact(ctx))
    }
}
