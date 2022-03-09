# rsa
Very simple RSA implementation using Chinese Remainder Theorem to decrypt messages with key parsing from JWK file.

## Setup
You need cargo to build and run this program.
You can install it using rustup: https://rustup.rs/

To run this project locally, compile it using cargo:
```
cargo build
````

## Code examples
Encrypt number 'number' using 'public_key' jwk file:
```
cargo run --bin without_crt -- --file 'public_key' --message 'number'
```

Decrypt encrypted number 'encrypted_number' using 'private_key' jwk file:
```
cargo run --bin crt -- --file 'private_key' --message 'encrypted_number'
