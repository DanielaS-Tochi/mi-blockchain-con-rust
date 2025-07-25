use crate::block::Block;
use crate::block::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, vec![], String::from("0"));
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let previous_block = self.blocks.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            transactions,
            previous_block.hash.clone(),
        );
        self.blocks.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.blocks.len() {
            let current_block = &self.blocks[i];
            let previous_block = &self.blocks[i - 1];

            // Verificar que el hash del bloque actual sea correcto
            let recalculated_hash = Block::calculate_hash(
                current_block.index,
                current_block.timestamp,
                &current_block.transactions, // Corrección: usar current_block.transactions
                &current_block.previous_hash, // Corrección: usar current_block.previous_hash
            );
            if recalculated_hash != current_block.hash {
                return false;
            }

            // Verificar que el hash del bloque anterior coincida
            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}