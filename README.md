# :crab: MINIGREP :crab:
A smaller version of grep written in Rust, build from accompanying knowledge provided in The Rust Programming Language.

## Usage
Minigrep uses two command line arguments for the search phrase and the target file, along with an optional env argument to specify case insensitive searches.

Search for a phrase: `cargo run {phrase} {path/to/file}`

Search for a phrase regardless of case: `CASE_INSENSITIVE=1 cargo run {phrase} {path/to/file}`

find the phrase `to` in the provided file `poem.txt`: `cargo run to poem.txt`

find the phrase `to` regardless of case in the same file: `CASE_INSENSITIVE=1 cargo run to poem.txt`

## Run unit tests
`cargo test` will run the accompanying unit tests for the two search functions.

## Some Things I learned / Implementation details
Used lifetimes to overcome ownership and reference restrictions, automated unit tests, processing cargo arguments, environment variables, std error outputs vs std outputs.
