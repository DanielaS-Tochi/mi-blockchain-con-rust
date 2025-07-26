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
