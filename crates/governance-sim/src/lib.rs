use core_contract::eco::EcoImpactMetrics;

/// Snapshot of an SNC rule configuration relevant for system‑level analysis.
#[derive(Clone, Debug)]
pub struct SncPolicySnapshot {
    pub min_knowledge_factor_open: f32,
    pub chat_issuance_slope: f32,
    pub eco_weight: f32,
}

/// Aggregate indicators returned by an Osireon‑style simulator.[web:136][web:149]
#[derive(Clone, Debug)]
pub struct SimulationOutcome {
    pub expected_neurorights_risk: f32,   // 0 = none, 1 = extreme
    pub environmental_justice_score: f32, // 0 = unjust, 1 = highly just
    pub trust_index: f32,                 // 0 = opaque, 1 = transparent
}

/// Trait for a policy simulator backend (Osireon, AEON, etc.).[web:136][web:137]
pub trait PolicySimulationBackend {
    fn evaluate_policy(
        &self,
        policy: &SncPolicySnapshot,
    ) -> Result<SimulationOutcome, String>;
}

/// Optional helper: combine EcoImpact into a simple global indicator.
pub fn eco_to_global_indicator(eco: &EcoImpactMetrics) -> f32 {
    eco.scalar()
}
