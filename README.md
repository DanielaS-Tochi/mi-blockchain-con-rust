# Mi Blockchain en Rust

![Rust](https://img.shields.io/badge/Rust-1.88.0-orange?logo=rust) ![License](https://img.shields.io/badge/License-MIT-blue)

Bienvenidos a **Mi Blockchain en Rust**, un proyecto educativo que implementa un blockchain simple en el lenguaje de programación Rust. Este proyecto está diseñado para enseñar conceptos fundamentales de blockchain, como bloques, transacciones, hashes y validación de cadenas, mientras se exploran las capacidades de Rust para desarrollar sistemas seguros y eficientes. Es ideal para estudiantes, desarrolladores principiantes y entusiastas de la tecnología blockchain que quieran prepararse para plataformas como Solana.

## 📖 Descripción del Proyecto

Este blockchain simple modela una cadena de bloques que almacena transacciones estructuradas. Cada bloque contiene un vector de transacciones (`Vec<Transaction>`), un índice, un timestamp, el hash del bloque anterior y su propio hash calculado con SHA-256. La cadena valida su integridad verificando que los hashes sean consistentes y que cada bloque esté correctamente ligado al anterior.

### Características Principales
- **Transacciones Estructuradas**: Cada transacción tiene un emisor (`sender`), receptor (`receiver`) y cantidad (`amount`), modelada con la estructura `Transaction`.
- **Validación de Cadena**: La función `is_valid` verifica la integridad de la cadena comprobando los hashes y los enlaces entre bloques.
- **Pruebas Unitarias**: Incluye pruebas en `src/block.rs` y `src/blockchain.rs` para garantizar la correcta creación de bloques y la validez de la cadena.
- **Serialización**: Usa `serde` para serializar transacciones, preparándote para conceptos de Solana como la serialización con `borsh`.
- **Diseño Modular**: Separado en módulos (`block` y `blockchain`) para mantener el código organizado y reutilizable.

### ¿Qué Aprenderás?
- Fundamentos de blockchain: bloques, hashes, y validación.
- Programación en Rust: structs, traits, serialización, y pruebas unitarias.
- Preparación para Solana: modelado de datos estructurados y conceptos de integridad.
- Buenas prácticas: documentación, pruebas, y modularidad.

## 🚀 Instalación

Para ejecutar el proyecto en tu máquina, sigue estos pasos:

1. **Instala Rust**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```
   Verifica la instalación con:
   ```bash
   rustc --version
   ```

2. **Clona el Repositorio**:
   ```bash
   git clone https://github.com/DanielaS-Tochi/mi-blockchain-con-rust.git
   cd mi-blockchain-con-rust
   ```

3. **Compila el Proyecto**:
   ```bash
   cargo build
   ```

4. **Ejecuta el Programa**:
   ```bash
   cargo run
   ```
   Esto creará un blockchain con un bloque génesis y dos bloques con transacciones de ejemplo, mostrando la cadena y su validez.

5. **Ejecuta las Pruebas**:
   ```bash
   cargo test
   ```
   Verás cinco pruebas pasando, verificando la creación de bloques y la validez de la cadena.

## 🛠 Estructura del Proyecto

| Archivo | Descripción |
|---------|-------------|
| `src/block.rs` | Define las estructuras `Transaction` y `Block`, y la lógica para calcular hashes. Incluye pruebas unitarias. |
| `src/blockchain.rs` | Define la estructura `Blockchain` y funciones para gestionar la cadena, como añadir bloques y validar. Incluye pruebas unitarias. |
| `src/main.rs` | Programa principal que crea y prueba el blockchain con transacciones de ejemplo. |
| `Cargo.toml` | Configura las dependencias (`sha2`, `chrono`, `serde`, `serde_json`). |

### Ejemplo de Salida
Al ejecutar `cargo run`, verás algo como:
```
Blockchain creado con bloque génesis: Block { index: 0, timestamp: ..., transactions: [], previous_hash: "0", hash: "..." }
Cadena de bloques: [Block { index: 0, transactions: [], ... }, Block { index: 1, transactions: [Transaction { sender: "Alice", receiver: "Bob", amount: 10 }], ... }, Block { index: 2, transactions: [Transaction { sender: "Bob", receiver: "Charlie", amount: 5 }], ... }]
¿Es válida la cadena? true
```

## 🧪 Pruebas Unitarias

El proyecto incluye pruebas unitarias para garantizar la robustez:
- **En `src/block.rs`**:
  - `test_block_creation`: Verifica que un bloque se cree con los datos correctos.
  - `test_hash_consistency`: Confirma que el hash calculado es consistente.
- **En `src/blockchain.rs`**:
  - `test_blockchain_creation`: Comprueba que la cadena inicia con un bloque génesis.
  - `test_add_block_and_validity`: Asegura que añadir bloques mantiene la validez.
  - `test_invalid_chain`: Detecta manipulaciones incorrectas en la cadena.

Ejecuta las pruebas con:
```bash
cargo test
```

## 📚 Cómo Contribuir

¡Tu aporte es bienvenido! Este proyecto está en una **fase de cierre temporal**, pero eso no significa que esté terminado. Estamos abiertos a nuevas ideas, mejoras y extensiones de la comunidad. Aquí hay algunas formas de contribuir:

1. **Reportar Errores**: Si encuentras problemas, crea un *issue* en el repositorio.
2. **Sugerir Mejoras**: Comparte ideas como:
   - Añadir persistencia en disco (guardar/cargar en JSON).
   - Implementar una interfaz CLI con `clap`.
   - Agregar prueba de trabajo (PoW) o validaciones de transacciones.
   - Integrar con conceptos de Solana (por ejemplo, modelado de cuentas).
3. **Enviar Código**: Haz un fork del repositorio, crea una rama, y envía un *pull request* con tus cambios. Asegúrate de:
   - Ejecutar `cargo fmt` para formatear el código.
   - Verificar con `cargo clippy` para detectar problemas.
   - Actualizar las pruebas si es necesario.

### Ideas para Futuras Extensiones
- **Persistencia**: Guardar la cadena en un archivo JSON.
- **CLI**: Interfaz para añadir transacciones desde la terminal.
- **Validaciones**: Verificar que las transacciones tengan saldo suficiente.
- **Red P2P**: Implementar nodos para simular una red descentralizada.
- **Tu Idea**: ¡Propón algo nuevo para llevar el proyecto más lejos!

## 🌟 Agradecimientos
- A la comunidad de Rust por sus bibliotecas como `sha2`, `chrono`, y `serde`.
- A los tutores y recursos en línea que inspiraron este proyecto.
- A los futuros contribuyentes que ayudarán a llevar este blockchain al siguiente nivel.

## 📜 Licencia
Este proyecto está licenciado bajo la [Licencia MIT](LICENSE). Siéntete libre de usarlo, modificarlo y compartirlo.

## 📩 Contacto
Si tienes preguntas, sugerencias o quieres colaborar, contáctame a través de [GitHub Issues](https://github.com/DanielaS-Tochi/mi-blockchain-con-rust/issues).

---

**Estado del Proyecto**: Este proyecto se encuentra en una fase de **cierre temporal** (julio de 2025), pero está abierto a contribuciones y nuevas iteraciones. ¡Únete y hagamos que este blockchain evolucione juntos! 🚀 **Creado con Rust, pasión y un toque de blockchain.**

---
Mi Blockchain
A simple blockchain implementation in Rust with a command-line interface (CLI) and a web interface built with Yew. This project is a practice and learning exercise to explore blockchain concepts, Rust programming, and web development with WebAssembly.
Features

Blockchain Core:
Blocks with timestamp, transactions, previous hash, hash, and nonce.
Proof-of-work mining with configurable difficulty (hashes start with three zeros).
Transaction validation and chain integrity check.


CLI:
Add transactions (sender, receiver, amount).
Mine blocks with a reward (Network -> Miner: 10).
Display the blockchain with block details and validity.
Persists state to blockchain.json.


Web Interface:
Form to add transactions.
Button to mine blocks.
Display of blockchain with block details (in-memory state).
Built with Yew and WebAssembly, served with Trunk.


Tests: 6 unit tests for block creation, hashing, and chain validation.

Project Structure

src/block.rs: Defines Transaction and Block structs, with methods for block creation, hashing, and mining.
src/blockchain.rs: Defines Blockchain struct, with methods for initialization, adding transactions, mining blocks, and validating the chain.
src/main.rs: CLI implementation using clap for commands (add-transaction, mine, show) and file-based persistence.
src/lib.rs: Web interface using Yew, with components for transaction input and blockchain display.
src/mod.rs: Module declarations for block and blockchain.
index.html: HTML template for the web interface with basic CSS styling.
Cargo.toml: Dependencies and configuration for Rust, including yew, wasm-bindgen, serde, and sha2.

Branches

main: Core blockchain with CLI, fully functional.
feature/web-ui: Adds the web interface with Yew and Trunk, maintaining CLI functionality.

Requirements

Rust (stable, wasm32-unknown-unknown target).
Trunk (cargo install trunk).
wasm-bindgen (cargo install wasm-bindgen-cli).

Installation

Clone the repository:git clone <your-repo-url>
cd mi_blockchain


Install Rust and required tools:rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli


Checkout desired branch:git checkout main  # or feature/web-ui



Usage
CLI
Run the CLI with:
cargo run --bin mi_blockchain -- <command>

Available commands:

add-transaction -s <sender> -r <receiver> -a <amount>: Add a transaction (e.g., Alice -> Bob: 5050).cargo run --bin mi_blockchain -- add-transaction -s Alice -r Bob -a 5050


mine: Mine a new block with pending transactions and a miner reward.cargo run --bin mi_blockchain -- mine


show: Display all blocks and chain validity.cargo run --bin mi_blockchain -- show



Web Interface

Ensure you're on the feature/web-ui branch:git checkout feature/web-ui


Serve the web app:trunk serve


Open http://localhost:8080 in your browser.
Use the form:
Enter Sender, Receiver, and Amount (e.g., Alice, Bob, 5050).
Click "Agregar Transacción" to add the transaction.
Click "Minar Bloque" to mine a block with pending transactions.
View blocks below, including timestamp, transactions, hashes, and nonce.



Tests
Run unit tests:
cargo test --bin mi_blockchain

Current Limitations

The web interface uses an in-memory blockchain, not synchronized with blockchain.json used by the CLI.
No API for full CLI-web synchronization (planned for future iterations).
Basic styling in the web interface; could be enhanced with more CSS or a frontend framework.

Future Improvements

Add a balance command to the CLI to show account balances.
Implement an API backend to synchronize the web interface with blockchain.json.
Enhance web styling with a CSS framework (e.g., Tailwind CSS).
Add a graphical representation of the blockchain in the web interface (e.g., Chart.js).
Upgrade to Rust edition = "2024" for modern features.

Contributing
This is a learning project, but feedback and contributions are welcome! To contribute:

Create a new branch for your feature:git checkout -b feature/<your-feature>


Commit changes and test thoroughly:cargo test --bin mi_blockchain
cargo run --bin mi_blockchain -- show
trunk serve


Push and create a pull request to main or feature/web-ui.

License
MIT License. Feel free to use and modify for learning purposes.
---