# Guía para Crear un Blockchain Simple en Rust

## Introducción
Este proyecto te guiará paso a paso para construir un blockchain simple en Rust, ideal para practicar los conceptos aprendidos en tus lecciones de Rust (variables, tipos, estructuras, manejo de errores, etc.) y prepararte para trabajar con Solana en el futuro. Un blockchain simple es una excelente manera de aprender sobre estructuras de datos, hashing y validación, mientras te introduces a los fundamentos de la tecnología blockchain.

## Objetivos del Proyecto
- **Practicar Rust**: Aplicar conceptos como structs, vectores, funciones, manejo de errores y posiblemente serialización.
- **Entender Blockchain**: Aprender cómo se estructuran los bloques, cómo se vinculan mediante hashes y cómo se verifica la integridad de la cadena.
- **Prepararte para Solana**: Familiarizarte con conceptos que serán útiles para desarrollar smart contracts en Solana, como el manejo seguro de datos.

## Requisitos Previos
- **Rust**: Instalado desde [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Conocimientos básicos**: Familiaridad con los conceptos de Rust cubiertos en tus lecciones (variables, structs, vectores, funciones, manejo de errores).
- **Dependencias**: Usaremos la biblioteca `sha2` para calcular hashes y `serde` para serialización (opcional).

## Estructura del Proyecto
El proyecto consistirá en un programa de línea de comandos que implementa un blockchain simple con las siguientes características:
- Crear bloques con un índice, timestamp, datos (transacciones), hash y hash del bloque anterior.
- Almacenar los bloques en una lista (cadena).
- Verificar la integridad de la cadena comprobando los hashes.
- (Opcional) Guardar la cadena en un archivo para persistencia.

### Estructura de Archivos
| Archivo | Descripción |
|---------|-------------|
| `src/main.rs` | Contiene el código principal para crear y probar el blockchain. |
| `src/block.rs` | Define la estructura `Block` y funciones relacionadas. |
| `src/blockchain.rs` | Define la estructura `Blockchain` y funciones para gestionar la cadena. |

## Pasos para Implementar el Blockchain

### 1. Configurar el Proyecto
1. Crea un nuevo proyecto Rust:
   ```bash
   cargo new mi_blockchain --bin
   cd mi_blockchain
   ```
2. Añade las dependencias necesarias en `Cargo.toml`:
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

### 2. Definir la Estructura de un Bloque
Crea un archivo `src/block.rs` para definir la estructura de un bloque y las funciones para calcular su hash.

```rust
use sha2::{Digest, Sha256};
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: i64,
    pub data: String, // Representa transacciones simples como un string
    pub previous_hash: String,
    pub hash: String,
}

impl Block {
    pub fn new(index: u32, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let hash = Block::calculate_hash(index, timestamp, &data, &previous_hash);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    pub fn calculate_hash(index: u32, timestamp: i64, data: &str, previous_hash: &str) -> String {
        let input = format!("{}{}{}{}", index, timestamp, data, previous_hash);
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
```

### 3. Definir la Estructura del Blockchain
Crea un archivo `src/blockchain.rs` para definir la estructura del blockchain y las funciones para gestionarlo.

```rust
use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, data: String) {
        let previous_block = self.blocks.last().unwrap();
        let new_block = Block::new(
            previous_block.index + 1,
            data,
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
                &current_block.data,
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

### 4. Implementar el Programa Principal
Modifica `src/main.rs` para probar el blockchain.

```rust
mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    println!("Blockchain creado con bloque génesis: {:?}", blockchain.blocks[0]);

    // Agregar algunos bloques
    blockchain.add_block("Transacción 1: Alice envía 10 a Bob".to_string());
    blockchain.add_block("Transacción 2: Bob envía 5 a Charlie".to_string());

    // Imprimir la cadena
    println!("Cadena de bloques: {:?}", blockchain.blocks);

    // Verificar la integridad
    println!("¿Es válida la cadena? {}", blockchain.is_valid());
}
```

### 5. Probar el Proyecto
Ejecuta el proyecto con:
```bash
cargo run
```
Esto creará un blockchain con un bloque génesis y dos bloques adicionales, y verificará su integridad.

### 6. Hacerlo Profesional y Accesible
- **Documentación**: Añade comentarios en español en el código para explicar cada parte.
- **Formato**: Usa `cargo fmt` para formatear el código y `cargo clippy` para detectar errores comunes.
- **Pruebas**: Añade pruebas unitarias en `src/block.rs` y `src/blockchain.rs` para verificar el cálculo de hashes y la validación de la cadena.
  ```rust
  #[cfg(test)]
  mod tests {
      use super::*;

      #[test]
      fn test_block_creation() {
          let block = Block::new(0, "test".to_string(), "0".to_string());
          assert_eq!(block.index, 0);
          assert_eq!(block.data, "test");
          assert_eq!(block.previous_hash, "0");
      }

      #[test]
      fn test_blockchain_validity() {
          let mut blockchain = Blockchain::new();
          blockchain.add_block("test block".to_string());
          assert!(blockchain.is_valid());
      }
  }
  ```
- **README.md**: Crea un archivo `README.md` que explique cómo instalar, ejecutar y probar el proyecto.

### 7. Extensiones Opcionales
- **Persistencia**: Guarda la cadena en un archivo JSON usando `serde_json`.
- **Prueba de Trabajo**: Implementa un mecanismo simple de prueba de trabajo (proof of work) donde el hash de un bloque debe comenzar con un número determinado de ceros.
- **Interfaz CLI**: Añade una interfaz de línea de comandos para interactuar con el blockchain (por ejemplo, agregar bloques o consultar la cadena).

## Recursos Adicionales
- **Tutoriales**:
  - [How to build a blockchain in Rust - LogRocket Blog](https://blog.logrocket.com/how-to-build-a-blockchain-in-rust/)
  - [Building a Blockchain with Rust: A Step-by-Step Guide - Rapid Innovation](https://www.rapidinnovation.io/post/how-to-build-a-blockchain-with-rust)
- **Repositorios en GitHub**:
  - [rust-blockchain](https://github.com/jean553/rust-blockchain)
  - [simple-rust-blockchain](https://github.com/zhanxdani/simple-rust-blockchain)
- **Documentación de Rust**:
  - [The Rust Programming Language](https://doc.rust-lang.org/book/)
  - [Rust By Example](https://doc.rust-lang.org/rust-by-example/)

## Consejos para el Éxito
- **Empieza simple**: Comienza con la implementación básica descrita arriba y luego añade funcionalidades según te sientas cómodo.
- **Experimenta**: Modifica el código para probar diferentes enfoques, como cambiar el formato de las transacciones o añadir validaciones adicionales.
- **Consulta la comunidad**: Únete a foros como [users.rust-lang.org](https://users.rust-lang.org/) o el subreddit [r/rust](https://www.reddit.com/r/rust/) para obtener ayuda y compartir tu progreso.

## Conclusión
Construir un blockchain simple en Rust es un proyecto ideal para consolidar tus habilidades en Rust y prepararte para trabajar con Solana. Te permitirá practicar conceptos clave de Rust mientras exploras los fundamentos de la tecnología blockchain. ¡Diviértete programando y buena suerte con tu proyecto! 🚀🦀