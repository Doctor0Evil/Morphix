#[derive(Clone, Debug)]
pub struct CorridorId(pub String);

/// Structured EcoImpact metrics used by SNC and CHAT scoring.
/// Values are normalized to [0,1], where 1.0 means best / least harm.
#[derive(Clone, Debug)]
pub struct EcoImpactMetrics {
    /// Energy / carbon efficiency (1.0 = minimal emissions per useful work).
    pub climate_score: f32,
    /// Biodiversity & habitat protection (1.0 = no expected harm).
    pub biodiversity_score: f32,
    /// Soil / water / air integrity (1.0 = no expected degradation).
    pub biosphere_score: f32,
    /// Urban / corridor friendliness (1.0 = respects corridor constraints).
    pub corridor_score: f32,
}

impl EcoImpactMetrics {
    pub fn scalar(&self) -> f32 {
        self.climate_score
            * self.biodiversity_score
            * self.biosphere_score
            * self.corridor_score
    }
}

/// Core SNC artifact; every contribution must declare corridor + EcoImpact.[file:69]
#[derive(Clone, Debug)]
pub struct NeuromorphArtifact {
    pub id: String,
    pub corridor_id: CorridorId,
    pub eco_impact: EcoImpactMetrics,
    /// Plaintext or structured representation of the content.
    pub summary: String,
}
