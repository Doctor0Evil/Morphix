use crate::eco_adapter::{EcoContext, EcoImpactAdapter, ImpactScore};
use crate::eco_adapter::sealed::Sealed;

/// GBIF-based biodiversity risk adapter (stubbed).
pub struct GbifRiskAdapter {
    /// Example of configuration: minimum occurrence count to flag
    /// high biodiversity sensitivity.
    pub high_risk_threshold: u32,
}

impl GbifRiskAdapter {
    pub fn new(high_risk_threshold: u32) -> Self {
        Self { high_risk_threshold }
    }
}

impl Sealed for GbifRiskAdapter {}

impl EcoImpactAdapter for GbifRiskAdapter {
    fn name(&self) -> &'static str {
        "gbif_risk_adapter_v1"
    }

    fn compute_impact(&self, ctx: &EcoContext) -> ImpactScore {
        // In a real implementation, you would:
        // 1. Call a GBIF client with ctx.taxon_or_feature and region_hint.
        // 2. Aggregate occurrences / red-list categories.
        // 3. Map to a normalized risk score in [0,1].[file:71]

        // Placeholder: high score if we have a taxon + region, else neutral.
        let has_specific_target =
            ctx.taxon_or_feature.is_some() && ctx.region_hint.is_some();

        if has_specific_target {
            ImpactScore::clamped(
                0.9,
                format!(
                    "High biodiversity sensitivity inferred for dataset={} taxon={:?} region={:?} (stub).",
                    ctx.dataset_id, ctx.taxon_or_feature, ctx.region_hint
                ),
            )
        } else {
            ImpactScore::clamped(
                0.5,
                format!(
                    "Neutral biodiversity impact for dataset={} (insufficient GBIF context, stub).",
                    ctx.dataset_id
                ),
            )
        }
    }
}
