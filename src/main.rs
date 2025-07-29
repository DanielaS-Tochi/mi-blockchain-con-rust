#![allow(dead_code)]

use clap::{Parser, Subcommand};
use mi_blockchain::block::Transaction;
use mi_blockchain::blockchain::Blockchain;
use std::fs;

mod block;
mod blockchain;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    AddTransaction {
        #[arg(short, long)]
        sender: String,
        #[arg(short, long)]
        receiver: String,
        #[arg(short, long)]
        amount: f64,
    },
    Mine,
    Show,
}

fn main() {
    let cli = Cli::parse();
    let mut blockchain = fs::read_to_string("blockchain.json")
        .map(|data| serde_json::from_str(&data).unwrap_or_else(|_| Blockchain::new()))
        .unwrap_or_else(|_| Blockchain::new());

    match cli.command {
        Some(Commands::AddTransaction { sender, receiver, amount }) => {
            blockchain.add_transaction(Transaction { sender, receiver, amount });
            save_blockchain(&blockchain);
            println!("Transacción agregada!");
            return;
        }
        Some(Commands::Mine) => {
            blockchain.add_block();
            save_blockchain(&blockchain);
            println!("Bloque minado!");
            return;
        }
        Some(Commands::Show) => {
            println!("Is blockchain valid? {}", blockchain.is_chain_valid());
            for (i, block) in blockchain.chain.iter().enumerate() {
                println!("Block #{}", i);
                println!("Timestamp: {}", block.timestamp);
                println!("Transactions: {:?}", block.transactions);
                println!("Previous Hash: {}", block.previous_hash);
                println!("Hash: {}", block.hash);
                println!("Nonce: {}", block.nonce);
            }
            return;
        }
        None => {}
    }

    // Modo interactivo
    loop {
        println!("Comandos: add-transaction <sender> <receiver> <amount>, mine, show, exit");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "exit" => break,
            "show" => {
                println!("Is blockchain valid? {}", blockchain.is_chain_valid());
                for (i, block) in blockchain.chain.iter().enumerate() {
                    println!("Block #{}", i);
                    println!("Timestamp: {}", block.timestamp);
                    println!("Transactions: {:?}", block.transactions);
                    println!("Previous Hash: {}", block.previous_hash);
                    println!("Hash: {}", block.hash);
                    println!("Nonce: {}", block.nonce);
                }
            }
            "mine" => {
                blockchain.add_block();
                save_blockchain(&blockchain);
                println!("Bloque minado!");
            }
            _ if input.starts_with("add-transaction") => {
                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() == 4 {
                    if let Ok(amount) = parts[3].parse::<f64>() {
                        blockchain.add_transaction(Transaction {
                            sender: parts[1].to_string(),
                            receiver: parts[2].to_string(),
                            amount,
                        });
                        save_blockchain(&blockchain);
                        println!("Transacción agregada!");
                    } else {
                        println!("Cantidad inválida");
                    }
                } else {
                    println!("Uso: add-transaction <sender> <receiver> <amount>");
                }
            }
            _ => println!("Comando inválido"),
        }
    }
}

fn save_blockchain(blockchain: &Blockchain) {
    let serialized = serde_json::to_string(blockchain).unwrap();
    fs::write("blockchain.json", serialized).unwrap();
}