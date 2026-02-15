use core_contract::eco::{EcoImpactMetrics, NeuromorphArtifact};
use core_contract::eco_source::EcoDataSource;
use core_contract::{SovereignNeuromorphContract, DistilledKnowledge, AccessClass, RoleTier};

/// Orchestrator now requires an EcoDataSource and uses its output
/// as the EcoImpact term in the knowledge-factor F_K.[file:69][file:55]
pub struct NeuromorphOrchestrator<C, E>
where
    C: SovereignNeuromorphContract,
    E: EcoDataSource,
{
    contract: C,
    eco_source: E,
}

impl<C, E> NeuromorphOrchestrator<C, E>
where
    C: SovereignNeuromorphContract,
    E: EcoDataSource,
{
    pub fn new(contract: C, eco_source: E) -> Self {
        Self { contract, eco_source }
    }

    pub fn distill_neuromorph_content(
        &self,
        role: RoleTier,
        artifact: NeuromorphArtifact,
        has_biophysical_signal: bool,
        uses_discipline_signals: bool,
        dual_empirical_formal_present: bool,
        uncertainty_exposed: bool,
    ) -> Result<DistilledKnowledge, String> {
        // 1. Sovereignty + neurorights checks (unchanged).
        if !self.contract.has_explicit_consent() {
            return Err("SNC violation: explicit consent required.".into());
        }
        if !self.contract.has_sovereign_abort_control() {
            return Err("SNC violation: sovereign abort control is mandatory.".into());
        }
        if !self.contract.is_discipline_personalized_and_non_coercive() {
            return Err(
                "SNC violation: discipline must be personalized and non-coercive.".into(),
            );
        }
        if !self.contract.forbids_downgrade_or_rollback() {
            return Err("SNC violation: downgrades/rollbacks are forbidden.".into());
        }

        // 2. CHAT eligibility: dual empirical + formal, uncertainty required.[file:55]
        if !dual_empirical_formal_present {
            return Err("CHAT-ineligible: missing dual empirical + formal linkage.".into());
        }
        if !uncertainty_exposed {
            return Err("CHAT-ineligible: uncertainty must be exposed.".into());
        }

        // 3. EcoImpact: refine the artifact’s eco_impact via pluggable source.
        let eco_refined: EcoImpactMetrics = self
            .eco_source
            .calculate(&artifact)
            .map_err(|e| format!("EcoImpact error: {e}"))?;

        // 4. Knowledge-factor components: V, R, E, N.[file:69]
        let validation = 0.9_f32;
        let reuse = 0.6_f32;
        let eco_impact = eco_refined.scalar().clamp(0.0, 1.0);
        let novelty = 0.7_f32;

        let fk = (validation * reuse * eco_impact * novelty).clamp(0.0, 1.0);

        // 5. Access class: ecological risk + neuromorphic sensitivity.[file:69]
        let access_class = if has_biophysical_signal || uses_discipline_signals {
            match role {
                RoleTier::Teacher | RoleTier::Mentor | RoleTier::Researcher => {
                    AccessClass::HighAutonomy
                }
                RoleTier::Learner => AccessClass::KnowledgeGated,
            }
        } else if fk >= 0.75 && eco_impact >= 0.8 {
            AccessClass::Open
        } else {
            AccessClass::KnowledgeGated
        };

        // 6. Delegate to existing DistilledKnowledge constructor.
        crate::distill_neuromorph_content_from_components(
            &self.contract,
            role,
            has_biophysical_signal,
            uses_discipline_signals,
            fk,
            access_class,
            self.eco_source.provenance_label(),
        )
    }
}
