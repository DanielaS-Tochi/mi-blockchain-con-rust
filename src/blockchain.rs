use crate::block::{Block, Transaction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(vec![], "0".to_string(), 2);
        Blockchain {
            chain: vec![genesis_block],
            pending_transactions: Vec::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction);
    }

    pub fn add_block(&mut self) {
        let previous_block = self.chain.last().unwrap();
        let new_block = Block::new(
            self.pending_transactions.clone(),
            previous_block.hash.clone(),
            2, // Dificultad 2
        );
        self.chain.push(new_block);
        self.pending_transactions.clear();
    }

    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() {
                return false;
            }

            if current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain.len(), 1);
        assert!(blockchain.pending_transactions.is_empty());
        assert_eq!(blockchain.chain[0].previous_hash, "0");
        assert!(blockchain.chain[0].hash.starts_with("00"));
    }

    #[test]
    fn test_blockchain_add_block() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 50.0,
        });
        blockchain.add_block();
        assert_eq!(blockchain.chain.len(), 2);
        assert!(blockchain.pending_transactions.is_empty());
        assert!(blockchain.is_chain_valid());
        assert!(blockchain.chain[1].hash.starts_with("00"));
    }

    #[test]
    fn test_blockchain_validity_invalid() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 50.0,
        });
        blockchain.add_block();
        blockchain.chain[1].hash = "invalid_hash".to_string();
        assert!(!blockchain.is_chain_valid());
    }
}