use std::time::SystemTime;

/// OCAP / CARE aligned community identifier.
#[derive(Clone, Debug)]
pub struct CommunityId(pub String);

/// FPIC status for a given proposal and community.[web:145][web:144]
#[derive(Clone, Debug)]
pub enum FpicStatus {
    Pending,
    Granted {
        timestamp: SystemTime,
        signed_by: Vec<String>, // community delegates, DID strings
    },
    Withheld {
        timestamp: SystemTime,
        reason: String,
    },
}

/// A governance proposal affecting SNC/CHAT rules or deployments.
#[derive(Clone, Debug)]
pub struct GovernanceProposal {
    pub id: String,
    pub title: String,
    pub description: String,
    /// Corridors, territories, or data scopes impacted.
    pub affected_corridors: Vec<String>,
    pub created_at: SystemTime,
}

/// Result of a community vote, suitable for recording on a permissioned ledger.[web:145][web:143]
#[derive(Clone, Debug)]
pub struct CommunityVoteResult {
    pub proposal_id: String,
    pub community_id: CommunityId,
    pub fpic_status: FpicStatus,
}

/// Minimal trait an FPIC / IDS layer must implement.
/// Backends can be blockchain, SSI registries, or other ledgers.[web:145][web:143]
pub trait CommunityGovernanceBackend {
    /// Look up current FPIC status for (proposal, community).
    fn get_fpic_status(
        &self,
        proposal_id: &str,
        community: &CommunityId,
    ) -> Result<FpicStatus, String>;

    /// Record a new FPIC decision result (e.g., after a community vote UI).
    fn record_fpic_result(
        &self,
        result: CommunityVoteResult,
    ) -> Result<(), String>;
}
