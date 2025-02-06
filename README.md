# valterm-arvidkr-cryptography

Basic fileserver which stores encrypted files in RAM

## Usage

Start server:

`cargo run -p server`

Upload file:

`cargo run -p client upload <input file> <key>`

Download file:

`cargo run -p client download <output file> <key> <id>`
