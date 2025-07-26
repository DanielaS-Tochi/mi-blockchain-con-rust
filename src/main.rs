mod block;
mod blockchain;

use crate::block::Transaction;
use crate::blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_transaction(Transaction {
        sender: "Alice".to_string(),
        receiver: "Bob".to_string(),
        amount: 50.0,
    });
    blockchain.add_transaction(Transaction {
        sender: "Bob".to_string(),
        receiver: "Charlie".to_string(),
        amount: 30.0,
    });

    blockchain.add_block();

    blockchain.add_transaction(Transaction {
        sender: "Charlie".to_string(),
        receiver: "Alice".to_string(),
        amount: 20.0,
    });

    blockchain.add_block();

    println!("Is blockchain valid? {}", blockchain.is_chain_valid());

    for (i, block) in blockchain.chain.iter().enumerate() {
        println!("Block #{}", i);
        println!("Timestamp: {}", block.timestamp);
        println!("Transactions: {:?}", block.transactions);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
        println!("Nonce: {}\n", block.nonce);
    }
}