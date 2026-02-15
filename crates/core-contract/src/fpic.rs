#![forbid(unsafe_code)]

use std::time::{Duration, SystemTime};

/// Minimal FPIC decision state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FpicStatus {
    Granted,
    Denied,
    Revoked,
}

/// CARE/OCAP-aligned consent lifetime.
/// `granted_at` is when FPIC was recorded; `max_age` encodes
/// “sufficiently in advance” + freshness windows.[file:69]
#[derive(Clone, Copy, Debug)]
pub struct ConsentLifetime {
    pub granted_at: SystemTime,
    pub max_age: Duration,
}

impl ConsentLifetime {
    pub fn is_fresh(&self, now: SystemTime) -> bool {
        match now.duration_since(self.granted_at) {
            Ok(delta) => delta <= self.max_age,
            Err(_) => false,
        }
    }
}

/// Runtime FPIC token with an always-available veto hook.
#[derive(Clone, Debug)]
pub struct FpicToken {
    status: FpicStatus,
    lifetime: ConsentLifetime,
}

impl FpicToken {
    pub fn new(granted_at: SystemTime, max_age: Duration) -> Self {
        Self {
            status: FpicStatus::Granted,
            lifetime: ConsentLifetime { granted_at, max_age },
        }
    }

    /// Participant or community can revoke at any time.
    pub fn veto(&mut self) {
        self.status = FpicStatus::Revoked;
    }

    pub fn status(&self, now: SystemTime) -> FpicStatus {
        if !self.lifetime.is_fresh(now) {
            return FpicStatus::Revoked;
        }
        self.status
    }
}
