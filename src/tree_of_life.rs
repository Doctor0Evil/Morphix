use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

use crate::config::Config;
use crate::ledger::{DeedEvent, Ledger};
use crate::utils::crypto::hash_json;

// TreeBranch defines the hierarchical structure of Tree-of-Life traits, grouped into branches for moral and biophysical evaluation.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TreeBranch {
    Branch1(Vec<TreeTrait>), // LOVE, GUILT, EVIL, FREEDOM, SPEECH, BLOOD
    Branch2(Vec<TreeTrait>), // WAR, JUSTICE, DECLARE, KARMA, COURAGE, NANO, LIFE, PREDICTION, FEAR, POWER, SPIRIT
    Branch3(Vec<TreeTrait>), // SOUL, JUDGEMENT, TIME, PEACE, TECH, EVOLVE, SMART, WISE, KNOWLEDGE
}

// TreeTrait represents individual traits in the Tree-of-Life, used for balancing assets and minting CHURCH tokens based on deeds.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TreeTrait {
    Love,
    Guilt,
    Evil,
    Freedom,
    Speech,
    Blood,
    War,
    Justice,
    Declare,
    Karma,
    Courage,
    Nano,
    Life,
    Prediction,
    Fear,
    Power,
    Spirit,
    Soul,
    Judgement,
    Time,
    Peace,
    Tech,
    Evolve,
    Smart,
    Wise,
    Knowledge,
}

impl TreeTrait {
    // Maps a trait to its branch for hierarchical evaluation.
    pub fn branch(&self) -> TreeBranch {
        match self {
            TreeTrait::Love | TreeTrait::Guilt | TreeTrait::Evil | TreeTrait::Freedom | TreeTrait::Speech | TreeTrait::Blood => {
                TreeBranch::Branch1(vec![self.clone()])
            }
            TreeTrait::War | TreeTrait::Justice | TreeTrait::Declare | TreeTrait::Karma | TreeTrait::Courage | TreeTrait::Nano | TreeTrait::Life | TreeTrait::Prediction | TreeTrait::Fear | TreeTrait::Power | TreeTrait::Spirit => {
                TreeBranch::Branch2(vec![self.clone()])
            }
            TreeTrait::Soul | TreeTrait::Judgement | TreeTrait::Time | TreeTrait::Peace | TreeTrait::Tech | TreeTrait::Evolve | TreeTrait::Smart | TreeTrait::Wise | TreeTrait::Knowledge => {
                TreeBranch::Branch3(vec![self.clone()])
            }
        }
    }

    // Computes a moral score for the trait based on deed context (e.g., positive for good-deeds, negative for harm).
    pub fn moral_score(&self, deed: &DeedEvent) -> i64 {
        let base_score = if deed.life_harm_flag { -10 } else { 10 };
        let ethics_penalty = deed.ethics_flags.len() as i64 * -5;
        base_score + ethics_penalty
    }
}

// TreeOfLife integrates Tree-of-Life traits with the ledger, evaluating deeds for CHURCH token minting and eco-grants.
#[derive(Clone)]
pub struct TreeOfLife {
    ledger: Ledger,
    config: Config,
    traits: Arc<RwLock<HashMap<String, TreeBranch>>>, // Event ID to evaluated branches
}

impl TreeOfLife {
    pub fn new(ledger: Ledger, config: Config) -> Self {
        TreeOfLife {
            ledger,
            config,
            traits: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Evaluates a deed against Tree-of-Life traits, updating the trait map and computing balances.
    pub async fn evaluate_deed(&self, event: &DeedEvent) -> Result<i64, String> {
        let mut traits = self.traits.write().await;
        let mut total_score = 0;

        // Example evaluation: Map deed_type to relevant traits and compute scores
        let relevant_traits = match event.deed_type.as_str() {
            "ecological_sustainability" => vec![TreeTrait::Life, TreeTrait::Peace, TreeTrait::Wise],
            "homelessness_relief" => vec![TreeTrait::Love, TreeTrait::Justice, TreeTrait::Karma],
            _ => vec![TreeTrait::Knowledge], // Default to learning/knowledge for unknown deeds
        };

        for trait in relevant_traits {
            let score = trait.moral_score(event);
            total_score += score;
            let branch = trait.branch();
            traits.insert(event.event_id.clone(), branch);
        }

        info!("Evaluated DeedEvent ID: {} with total moral score: {}", event.event_id, total_score);
        Ok(total_score)
    }

    // Computes eco-grants based on accumulated trait scores for CHURCH token minting.
    pub async fn compute_eco_grant(&self) -> i64 {
        let traits = self.traits.read().await;
        let mut grant = 0;

        for (_, branch) in traits.iter() {
            match branch {
                TreeBranch::Branch1(traits) => grant += traits.len() as i64 * 5, // Positive for core values
                TreeBranch::Branch2(traits) => grant += traits.len() as i64 * 10, // Higher for action-oriented
                TreeBranch::Branch3(traits) => grant += traits.len() as i64 * 15, // Highest for wisdom/evolution
            }
        }

        grant * self.config.eco_grant_multiplier as i64
    }
}

// Unit tests for Tree-of-Life integration.
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::config::Config;
    use crate::ledger::{DeedEvent, Ledger};

    #[tokio::test]
    async fn test_tree_of_life_evaluation() {
        let config = Config::default();
        let ledger = Ledger::new(config.clone());
        let tree = TreeOfLife::new(ledger, config);

        let event = DeedEvent::new(
            "genesis".to_string(),
            "actor1".to_string(),
            vec!["target1".to_string()],
            "ecological_sustainability".to_string(),
            vec!["math_science_education".to_string()],
            HashMap::new(),
            vec![],
            false,
        );

        let score = tree.evaluate_deed(&event).await.unwrap();
        assert!(score > 0);

        let grant = tree.compute_eco_grant().await;
        assert!(grant > 0);
    }
}
