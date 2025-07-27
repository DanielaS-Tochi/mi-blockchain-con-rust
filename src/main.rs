mod block;
mod blockchain;

use crate::block::Transaction;
use crate::blockchain::Blockchain;
use clap::{Parser, Subcommand};
use std::io;

#[derive(Parser)]
#[command(name = "mi_blockchain")]
#[command(about = "Una blockchain simple en Rust", long_about = None)]
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
    let mut blockchain = Blockchain::new();

    loop {
        println!("\n=== Mi Blockchain ===");
        println!("Comandos: add-transaction, mine, show, exit");
        println!("Ingresá un comando:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer");
        let input = input.trim();

        if input == "exit" {
            println!("Chau!");
            break;
        }

        let args: Vec<&str> = input.split_whitespace().collect();
        if args.is_empty() {
            continue;
        }

        let cli = match Cli::try_parse_from(std::iter::once("mi_blockchain").chain(args.iter().copied())) {
            Ok(cli) => cli,
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        };

        match cli.command {
            Some(Commands::AddTransaction { sender, receiver, amount }) => {
                blockchain.add_transaction(Transaction { sender, receiver, amount });
                println!("Transacción agregada!");
            }
            Some(Commands::Mine) => {
                blockchain.add_block();
                println!("Bloque minado!");
            }
            Some(Commands::Show) => {
                println!("Is blockchain valid? {}", blockchain.is_chain_valid());
                for (i, block) in blockchain.chain.iter().enumerate() {
                    println!("Block #{i}");
                    println!("Timestamp: {}", block.timestamp);
                    println!("Transactions: {:?}", block.transactions);
                    println!("Previous Hash: {}", block.previous_hash);
                    println!("Hash: {}", block.hash);
                    println!("Nonce: {}\n", block.nonce);
                }
            }
            None => {
                println!("Comando no reconocido. Usá: add-transaction, mine, show, exit");
            }
        }
    }
}