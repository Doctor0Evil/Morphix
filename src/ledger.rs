use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use tracing::info;
use uuid::Uuid;

use crate::config::Config;
use crate::utils::crypto::hash_json;

// DeedEvent represents a single morally relevant action in the neuromorphic microspace.
// It is designed as an immutable, hash-linked unit for tamper-evident auditing.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeedEvent {
    pub event_id: String, // UUID as string for global uniqueness
    pub timestamp: u64, // Unix epoch in seconds
    pub prev_hash: String, // SHA-256 hash of previous event's self_hash
    pub self_hash: String, // SHA-256 hash of this event's serialized JSON
    pub actor_id: String, // Unique ID of the agent performing the deed
    pub target_ids: Vec<String>, // IDs of agents affected by the deed
    pub deed_type: String, // High-level classification (e.g., "ecological_sustainability")
    pub tags: Vec<String>, // Keywords for categorization (e.g., "math_science_education")
    pub context_json: HashMap<String, serde_json::Value>, // Optional evidence or parameters
    pub ethics_flags: Vec<String>, // Violations of ALN ethics or RoH breaches
    pub life_harm_flag: bool, // True if the deed harmed a living creature or lifeform
}

impl DeedEvent {
    // Creates a new DeedEvent with automatic hashing and timestamping.
    pub fn new(
        prev_hash: String,
        actor_id: String,
        target_ids: Vec<String>,
        deed_type: String,
        tags: Vec<String>,
        context_json: HashMap<String, serde_json::Value>,
        ethics_flags: Vec<String>,
        life_harm_flag: bool,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let mut event = DeedEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp,
            prev_hash,
            self_hash: String::new(), // Placeholder, will be computed
            actor_id,
            target_ids,
            deed_type,
            tags,
            context_json,
            ethics_flags,
            life_harm_flag,
        };

        // Compute self_hash based on serialized JSON (excluding self_hash field)
        let serialized = serde_json::to_string(&event).expect("Serialization failed");
        event.self_hash = hash_json(&serialized);

        event
    }

    // Validates the event's integrity against its self_hash and prev_hash.
    pub fn validate(&self, expected_prev_hash: &str) -> bool {
        let serialized = serde_json::to_string(self).expect("Serialization failed");
        let computed_hash = hash_json(&serialized);
        if computed_hash != self.self_hash {
            info!("Self-hash mismatch for event ID: {}", self.event_id);
            return false;
        }
        if self.prev_hash != expected_prev_hash {
            info!("Prev-hash mismatch for event ID: {}", self.event_id);
            return false;
        }
        true
    }
}

// Ledger manages the chain of DeedEvents, ensuring append-only immutability.
#[derive(Clone)]
pub struct Ledger {
    events: Arc<RwLock<Vec<DeedEvent>>>,
    config: Config,
}

impl Ledger {
    pub fn new(config: Config) -> Self {
        Ledger {
            events: Arc::new(RwLock::new(Vec::new())),
            config,
        }
    }

    // Appends a new DeedEvent to the ledger after validation.
    pub async fn append(&self, event: DeedEvent) -> Result<(), String> {
        let mut events = self.events.write().await;
        let expected_prev_hash = if events.is_empty() {
            "genesis".to_string() // Initial hash for the chain
        } else {
            events.last().unwrap().self_hash.clone()
        };

        if !event.validate(&expected_prev_hash) {
            return Err("Event validation failed".to_string());
        }

        events.push(event);
        info!("Appended DeedEvent ID: {} with type: {}", events.last().unwrap().event_id, events.last().unwrap().deed_type);
        Ok(())
    }

    // Computes metrics over the ledger for CHURCH token minting.
    pub async fn compute_metrics(&self) -> Metrics {
        let events = self.events.read().await;
        let mut good_deeds = 0;
        let mut harm_flags = 0;

        for event in events.iter() {
            if event.life_harm_flag {
                harm_flags += 1;
            } else {
                good_deeds += 1;
            }
        }

        Metrics {
            total_events: events.len() as u64,
            good_deeds,
            harm_flags,
            balance: Balance { church_tokens: good_deeds * self.config.token_mint_rate },
        }
    }
}

// Metrics for ledger analysis, supporting eco_grants and debt_ceiling adjustments.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Metrics {
    pub total_events: u64,
    pub good_deeds: u64,
    pub harm_flags: u64,
    pub balance: Balance,
}

// Balance represents accumulated CHURCH tokens from good deeds.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Balance {
    pub church_tokens: u64,
}

// Unit tests for the ledger module.
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_ledger_append_and_validate() {
        let config = Config::default();
        let ledger = Ledger::new(config);

        let event1 = DeedEvent::new(
            "genesis".to_string(),
            "actor1".to_string(),
            vec!["target1".to_string()],
            "ecological_sustainability".to_string(),
            vec!["math_science_education".to_string()],
            HashMap::new(),
            vec![],
            false,
        );

        assert!(ledger.append(event1.clone()).await.is_ok());

        let event2 = DeedEvent::new(
            event1.self_hash.clone(),
            "actor2".to_string(),
            vec!["target2".to_string()],
            "homelessness_relief".to_string(),
            vec!["civic_duty".to_string()],
            HashMap::new(),
            vec![],
            false,
        );

        assert!(ledger.append(event2).await.is_ok());

        let metrics = ledger.compute_metrics().await;
        assert_eq!(metrics.good_deeds, 2);
        assert_eq!(metrics.harm_flags, 0);
    }
}
