use core_contract::eco::{EcoImpactMetrics, NeuromorphArtifact};
use core_contract::eco_source::EcoDataSource;

/// Stub implementation: in production, call GBIF / planetary APIs.[file:71][file:69]
pub struct GbifEcoSource;

impl EcoDataSource for GbifEcoSource {
    fn calculate(&self, artifact: &NeuromorphArtifact) -> Result<EcoImpactMetrics, String> {
        // Placeholder logic: derive scores from corridor_id prefix, then clamp.
        let corridor = artifact.corridor_id.0.as_str();

        let (climate, biodiversity, biosphere, corridor_score) = if corridor.starts_with("urban") {
            (0.7_f32, 0.5_f32, 0.6_f32, 0.8_f32)
        } else if corridor.starts_with("protected") {
            (0.9_f32, 0.9_f32, 0.95_f32, 0.9_f32)
        } else {
            (0.8_f32, 0.7_f32, 0.7_f32, 0.7_f32)
        };

        Ok(EcoImpactMetrics {
            climate_score: climate.clamp(0.0, 1.0),
            biodiversity_score: biodiversity.clamp(0.0, 1.0),
            biosphere_score: biosphere.clamp(0.0, 1.0),
            corridor_score: corridor_score.clamp(0.0, 1.0),
        })
    }

    fn provenance_label(&self) -> &'static str {
        "stub-gbif-eco-source-v1"
    }
}
