use core_contract::eco::{CorridorId, EcoImpactMetrics, NeuromorphArtifact};
use core_contract::{DefaultSovereignNeuromorphContract, RoleTier};
use eco_gbif::GbifEcoSource;
use orchestration::NeuromorphOrchestrator;

fn main() {
    let contract = DefaultSovereignNeuromorphContract::new(true, true, true);
    let eco_source = GbifEcoSource;
    let orchestrator = NeuromorphOrchestrator::new(contract, eco_source);

    let artifact = NeuromorphArtifact {
        id: "artifact-001".to_string(),
        corridor_id: CorridorId("protected-desert-phoenix".to_string()),
        eco_impact: EcoImpactMetrics {
            climate_score: 1.0,
            biodiversity_score: 1.0,
            biosphere_score: 1.0,
            corridor_score: 1.0,
        },
        summary: "Example neuromorph research turn for Phoenix corridor.".to_string(),
    };

    let result = orchestrator.distill_neuromorph_content(
        RoleTier::Learner,
        artifact,
        /* has_biophysical_signal */ true,
        /* uses_discipline_signals */ true,
        /* dual_empirical_formal_present */ true,
        /* uncertainty_exposed */ true,
    );

    match result {
        Ok(dk) => {
            println!(
                "Distilled knowledge: F_K={:.3}, access={:?}, hex={}, eco‑safe={}",
                dk.knowledge_factor,
                dk.access_class,
                dk.hex_stamp,
                dk.neurorights_compliant,
            );
        }
        Err(err) => eprintln!("SNC refused: {err}"),
    }
}
