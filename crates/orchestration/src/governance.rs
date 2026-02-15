use crate::NeuromorphOrchestrator;
use governance_local::{CommunityGovernanceBackend, CommunityId, FpicStatus};
use governance_sim::{PolicySimulationBackend, SncPolicySnapshot};

/// Guard a proposed SNC / CHAT policy change behind FPIC + global simulation.[web:145][web:146]
pub fn validate_policy_change<G, S>(
    governance: &G,
    simulator: &S,
    proposal_id: &str,
    affected_communities: &[CommunityId],
    snapshot: &SncPolicySnapshot,
) -> Result<(), String>
where
    G: CommunityGovernanceBackend,
    S: PolicySimulationBackend,
{
    // 1. FPIC: every affected community must have Granted status.[web:145][web:143]
    for community in affected_communities {
        match governance.get_fpic_status(proposal_id, community)? {
            FpicStatus::Granted { .. } => {}
            FpicStatus::Pending => {
                return Err(format!(
                    "Policy blocked: FPIC still pending for community {:?}.",
                    community.0
                ));
            }
            FpicStatus::Withheld { reason, .. } => {
                return Err(format!(
                    "Policy blocked: FPIC withheld by community {:?}: {}",
                    community.0, reason
                ));
            }
        }
    }

    // 2. Osireon‑style simulation: reject clearly unsafe futures.[web:136][web:149][web:146]
    let outcome = simulator.evaluate_policy(snapshot)?;
    if outcome.expected_neurorights_risk > 0.3 {
        return Err("Policy blocked: neurorights risk too high in simulation.".into());
    }
    if outcome.environmental_justice_score < 0.6 {
        return Err("Policy blocked: environmental justice score too low.".into());
    }

    Ok(())
}
