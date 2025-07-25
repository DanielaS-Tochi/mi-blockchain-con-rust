use sha2::{Digest, Sha256};
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u32, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let hash = Block::calculate_hash(index, timestamp, &transactions, &previous_hash);
        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash,
        }
    }

    pub fn calculate_hash(index: u32, timestamp: i64, transactions: &[Transaction], previous_hash: &str) -> String {
        let transactions_str = serde_json::to_string(transactions).unwrap_or_default();
        let input = format!("{}{}{}{}", index, timestamp, transactions_str, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let tx = vec![Transaction {
            sender: String::from("Alice"),
            receiver: String::from("Bob"),
            amount: 10,
        }];
        let block = Block::new(0, tx.clone(), String::from("0"));
        assert_eq!(block.index, 0);
        assert_eq!(block.transactions, tx);
        assert_eq!(block.previous_hash, String::from("0"));
        assert!(!block.hash.is_empty());
    }

    #[test]
    fn test_hash_consistency() {
        let tx = vec![Transaction {
            sender: String::from("Alice"),
            receiver: String::from("Bob"),
            amount: 10,
        }];
        let block = Block::new(0, tx.clone(), String::from("0"));
        let recalculated_hash = Block::calculate_hash(
            block.index,
            block.timestamp,
            &block.transactions,
            &block.previous_hash,
        );
        assert_eq!(block.hash, recalculated_hash);
    }
}