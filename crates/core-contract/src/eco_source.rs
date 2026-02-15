use crate::eco::{EcoImpactMetrics, NeuromorphArtifact};

/// Pluggable provider interface for EcoImpact metrics.[file:71][file:69]
pub trait EcoDataSource {
    /// Compute refined EcoImpact for a given artifact.
    /// Implementations may call external APIs, local models, or simulators.
    fn calculate(&self, artifact: &NeuromorphArtifact) -> Result<EcoImpactMetrics, String>;

    /// Optional human-readable provenance label (e.g., "GBIF+Copernicus v1").
    fn provenance_label(&self) -> &'static str;
}
