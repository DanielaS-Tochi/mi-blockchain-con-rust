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
            sender: String::from("Alice"),
            receiver: String::from("Bob"),
            amount: 10,
        },
    ]);
    blockchain.add_block(vec![
        Transaction {
            sender: String::from("Bob"),
            receiver: String::from("Charlie"),
            amount: 5,
        },
    ]);

    // Imprimir la cadena
    println!("Cadena de bloques: {:?}", blockchain.blocks);

    // Verificar la integridad
    println!("¿Es válida la cadena? {}", blockchain.is_valid());
}