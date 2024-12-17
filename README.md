# Kuti CryptoPad

Kuti CryptoPad is a simple and efficient tool for protecting text files, designed to help you secure your sensitive information. Written in Rust for its performance, reliability, and memory safety features, this project offers a straightforward way to create, encrypt, and decrypt text files, ensuring that your data remains safe from unauthorized access.

## Features

- **Encryption**: Securely encrypt your text files using industry-standard algorithms. See encryption details below.
- **Decryption**: Easily decrypt them with your passkey when needed.
- **User-Friendly**: Simple Slint based user interface for ease of use.
- **Cross-Platform**: Thanks to Slint UI, this app works on multiple operating systems including Windows, macOS, and Linux.
- **Transparency**: Open-source code and libraries available for audit by anyone.

## Installation

To install Kuti CryptoPad, clone the repository and navigate to the project directory:

```bash
git clone https://github.com/yourusername/kuti-cryptopad.git
cd kuti-cryptopad
```

## Usage

To build:

```bash
cargo build
```
or 
```bash
cargo build --release
```


To run the app:

```bash
cargo run
```

## Encryption Details

Kuti CryptoPad uses industry-standard encryption techniques to secure your text files:

### AES-256 Encryption in CTR Mode

The application employs AES-256 (Advanced Encryption Standard with a 256-bit key size) in CTR (Counter) mode. AES-256 is a symmetric encryption algorithm widely recognized for its security and efficiency. CTR mode turns block cipher operations into a stream cipher, allowing for encryption of data of arbitrary length.

### Key Derivation with PBKDF2 and HMAC-SHA256

User-provided passkeys are converted into secure encryption keys using PBKDF2 (Password-Based Key Derivation Function 2) with HMAC-SHA256 as the pseudorandom function. This process involves:

- **Salt**: A unique, randomly generated 16-byte salt is used for each encryption. Salts protect against dictionary attacks and ensure that the same passkey generates different keys when used multiple times.
- **Iterations**: The key derivation uses 100,000 iterations. A higher number of iterations increases the computational work required to derive the key, enhancing security against brute-force attacks.
- **Random Initialization Vector (IV)**: A randomly generated 16-byte IV is created for each encryption operation. The IV ensures that encrypting the same plaintext with the same key results in different ciphertexts, adding an additional layer of security.

### Secure Data Concatenation

The encryption process outputs a concatenation of the salt, IV, and ciphertext in the following format:

```
salt || IV || ciphertext
```

This design allows the decryption function to extract the necessary parameters without needing external information.

### Base64 Encoding

The combined data (salt, IV, ciphertext) is base64-encoded to produce an ASCII string. This encoding facilitates safe and convenient storage or transmission of the encrypted data, especially in text-based formats.

### Decryption Process

During decryption, the application:

1. Extracts the salt and IV from the beginning of the encrypted data.
2. Uses the same PBKDF2 parameters (passkey, extracted salt, and iteration count) to derive the encryption key.
3. Decrypts the ciphertext using AES-256 in CTR mode with the derived key and extracted IV.

By implementing these cryptographic practices, Kuti CryptoPad ensures that your sensitive information remains confidential and protected against unauthorized access.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For any questions or feedback, please open an issue on the GitHub repository.

## Disclaimer

Please note that if you forget your passkey or mistype it during the creation of the encrypted file, the data cannot be recovered. The encryption process is designed to be secure and does not include any backdoor or recovery mechanism. Ensure that you store your passkey securely and verify it during the encryption process to avoid data loss.