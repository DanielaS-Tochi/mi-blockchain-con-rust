use chrono::Utc;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let mut block = Block {
            timestamp: Utc::now().timestamp(),
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let input = format!(
            "{}{}{}{}",
            self.timestamp,
            serde_json::to_string(&self.transactions).unwrap(),
            self.previous_hash,
            self.nonce
        );
        hasher.update(input);
        let result = hasher.finalize();
        format!("{result:x}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let transactions = vec![Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 50.0,
        }];
        let previous_hash = "0".to_string();
        let block = Block::new(transactions.clone(), previous_hash.clone());

        assert_eq!(block.transactions, transactions);
        assert_eq!(block.previous_hash, previous_hash);
        assert!(!block.hash.is_empty());
        assert_eq!(block.nonce, 0);
    }

    #[test]
    fn test_block_hash() {
        let transactions = vec![Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 50.0,
        }];
        let previous_hash = "0".to_string();
        let block = Block::new(transactions.clone(), previous_hash.clone());

        let expected_hash = block.calculate_hash();
        assert_eq!(block.hash, expected_hash);
    }
}
