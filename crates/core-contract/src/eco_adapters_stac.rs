use crate::eco_adapter::{EcoContext, EcoImpactAdapter, ImpactScore};
use crate::eco_adapter::sealed::Sealed;

/// STAC / Planetary Computer-based eco adapter (stubbed).
pub struct StacEcoAdapter {
    /// Base STAC API URL, e.g. Planetary Computer or other STAC server.[web:148]
    pub stac_api_url: String,
}

impl StacEcoAdapter {
    pub fn new(stac_api_url: impl Into<String>) -> Self {
        Self {
            stac_api_url: stac_api_url.into(),
        }
    }
}

impl Sealed for StacEcoAdapter {}

impl EcoImpactAdapter for StacEcoAdapter {
    fn name(&self) -> &'static str {
        "stac_eco_adapter_v1"
    }

    fn compute_impact(&self, ctx: &EcoContext) -> ImpactScore {
        // In a real implementation:
        // - Use stac_client or a custom async client to query items
        //   intersecting ctx.region_hint for ctx.dataset_id.[web:148]
        // - Derive impact metrics from bands, time series, etc.
        //
        // For now we just emit a low-risk placeholder with explanation.

        ImpactScore::clamped(
            0.3,
            format!(
                "Low-to-moderate eco impact inferred from STAC dataset={} at {} (stub).",
                ctx.dataset_id,
                self.stac_api_url
            ),
        )
    }
}
