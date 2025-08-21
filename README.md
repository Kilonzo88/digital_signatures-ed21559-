# Ed25519 Digital Signatures Project

A Rust implementation of digital signatures using the Ed25519 elliptic curve cryptography algorithm. This project demonstrates how to generate key pairs, sign messages, and verify signatures securely.

## Features

- **Secure Key Generation**: Uses cryptographically secure random number generation via `OsRng`
- **Digital Signatures**: Sign messages with Ed25519 algorithm
- **Signature Verification**: Verify message authenticity and integrity
- **Hex Encoding**: Convert binary keys and signatures to human-readable hex format
- **Production Ready**: Uses industry-standard cryptographic libraries

## Technology Stack

- **Rust** - Systems programming language
- **ed25519-dalek** - Ed25519 digital signatures implementation
- **rand** - Cryptographically secure random number generation
- **hex** - Hexadecimal encoding utilities

## Installation

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Create new project**:
   ```bash
   cargo new ed25519-signatures
   cd ed25519-signatures
   ```

3. **Add dependencies**:
   ```bash
   cargo add ed25519-dalek
   cargo add rand --features std_rng
   cargo add hex
   ```

## Usage

### Basic Example

```rust
use ed25519_dalek::{SigningKey, Signature, Signer, Verifier, VerifyingKey};
use rand::rngs::OsRng;

fn main() {
    // Generate key pair
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let public_key = signing_key.verifying_key();

    // Sign a message
    let message = b"Hello, world!";
    let signature = signing_key.sign(message);

    // Convert to hex for storage/transmission
    let public_key_hex = hex::encode(public_key.to_bytes());
    let signature_hex = hex::encode(signature.to_bytes());
    
    println!("Public Key: {}", public_key_hex);
    println!("Signature: {}", signature_hex);

    // Verify signature
    let is_valid = public_key.verify(message, &signature).is_ok();
    println!("Signature valid: {}", is_valid);
}
```

### Output Format

- **Public Key**: 32 bytes (64 hex characters)
- **Signature**: 64 bytes (128 hex characters)

Example output:
```
Public Key: 59a86f57ab97464daac427c782ac4ac32bfcdf861331e7641cf771ab14b96370
Signature: 8d6183f4a8e7c94a1c2b8e3d5f6a7c9b1d4e5f6a7c8b9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2c3d4e5f6a7b8c9d0e1f2
Signature valid: true
```

## Project Structure

```
ed25519-signatures/
├── Cargo.toml          # Dependencies and project configuration
├── Cargo.lock          # Exact dependency versions
├── src/
│   └── main.rs         # Main application code
└── target/             # Build artifacts (gitignored)
```

## Security Features

- **Cryptographically Secure RNG**: Uses operating system's entropy source (`/dev/urandom`, `BCryptGenRandom`)
- **Constant-time Operations**: Prevents timing attacks
- **Memory Safety**: Rust's ownership system prevents common vulnerabilities
- **No Heap Allocation**: Sensitive data kept on stack when possible

## Common Use Cases

- **API Authentication**: Sign requests to verify authenticity
- **Software Updates**: Verify update package integrity
- **Blockchain Transactions**: Sign and verify cryptocurrency transactions
- **Secure Messaging**: End-to-end encrypted communication
- **Document Signing**: Digital notarization and verification

## Development

### Building the Project

```bash
cargo build
```

### Running the Example

```bash
cargo run
```

### Running Tests

```bash
cargo test
```

### Building for Release

```bash
cargo build --release
```

## Dependencies

- `ed25519-dalek = "2.0.0"` - Ed25519 digital signatures
- `rand = "0.8.5"` - Random number generation
- `hex = "0.4.3"` - Hexadecimal encoding

## License

This project is open source and available under the [MIT License](LICENSE).

## Contributing

1. Fork the project
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Resources

- [Ed25519 Documentation](https://docs.rs/ed25519-dalek/latest/ed25519_dalek/)
- [Rust Cryptography](https://github.com/RustCrypto)
- [Ed25519: High-Speed High-Security Signatures](https://ed25519.cr.yp.to/)

