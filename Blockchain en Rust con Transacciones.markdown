# Blockchain Simple en Rust con Transacciones

## Descripción
Este proyecto implementa un blockchain simple en Rust, actualizado para usar una estructura `Transaction` que modela transacciones con emisor, receptor y cantidad. Los bloques ahora contienen un vector de transacciones (`Vec<Transaction>`) en lugar de un simple `String`. Este cambio hace que el proyecto sea más estructurado y relevante para aprender conceptos aplicables a Solana.

## Estructura de Archivos
| Archivo | Descripción |
|---------|-------------|
| `src/block.rs` | Define las estructuras `Transaction` y `Block`, y la lógica para calcular hashes. |
| `src/blockchain.rs` | Define la estructura `Blockchain` y funciones para gestionar la cadena. |
| `src/main.rs` | Programa principal que crea y prueba el blockchain con transacciones. |

## Contenido de los Archivos

### `src/block.rs`
```rust
use sha2::{Digest, Sha256};
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
```

### `src/blockchain.rs`
```rust
use crate::block::Block;
use crate::block::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, vec![], "0".to_string());
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
                &current_block.transactions,
                &current_block.previous_hash,
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
```

### `src/main.rs`
```rust
mod block;
mod blockchain;

use blockchain::Blockchain;
use block::Transaction;

fn main() {
    let mut blockchain = Blockchain::new();
    println!("Blockchain creado con bloque génesis: {:?}", blockchain.blocks[0]);

    // Agregar bloques con transacciones
    blockchain.add_block(vec![
        Transaction {
            sender: "Alice".to_string(),
            receiver: "Bob".to_string(),
            amount: 10,
        },
    ]);
    blockchain.add_block(vec![
        Transaction {
            sender: "Bob".to_string(),
            receiver: "Charlie".to_string(),
            amount: 5,
        },
    ]);

    // Imprimir la cadena
    println!("Cadena de bloques: {:?}", blockchain.blocks);

    // Verificar la integridad
    println!("¿Es válida la cadena? {}", blockchain.is_valid());
}
```

## Instrucciones
1. Reemplaza el contenido de `src/block.rs`, `src/blockchain.rs` y `src/main.rs` con los códigos proporcionados.
2. Asegúrate de que `Cargo.toml` incluya las dependencias necesarias:
   ```toml
   [package]
   name = "mi_blockchain"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   sha2 = "0.10"
   chrono = "0.4"
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   ```
3. Compila y ejecuta:
   ```bash
   cargo run
   ```
4. Verifica la salida, que debería mostrar bloques con transacciones estructuradas y confirmar que la cadena es válida.

## Salida Esperada
```
Blockchain creado con bloque génesis: Block { index: 0, timestamp: ..., transactions: [], previous_hash: "0", hash: "..." }
Cadena de bloques: [Block { index: 0, transactions: [], ... }, Block { index: 1, transactions: [Transaction { sender: "Alice", receiver: "Bob", amount: 10 }], ... }, Block { index: 2, transactions: [Transaction { sender: "Bob", receiver: "Charlie", amount: 5 }], ... }]
¿Es válida la cadena? true
```