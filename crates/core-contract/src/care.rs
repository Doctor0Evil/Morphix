#[derive(Clone, Debug)]
pub struct CareAttestation {
    pub collective_benefit: bool,
    pub authority_to_control: bool,
    pub responsibility: bool,
    pub ethics: bool,
    /// Opaque on-chain or ALN proof (e.g., hash, signature).
    pub proof_ref: Option<String>,
}

impl CareAttestation {
    pub fn is_fully_care_aligned(&self) -> bool {
        self.collective_benefit
            && self.authority_to_control
            && self.responsibility
            && self.ethics
    }
}

/// Object-safe trait for CARE-aware provenance.
pub trait CareAttestable {
    fn care_attestation(&self) -> &CareAttestation;
}
