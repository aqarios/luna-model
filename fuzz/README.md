# Fuzzing

Coverage-guided fuzzing of the code that reads outside input. 
It needs nightly Rust, which `rust-toolchain.toml` selects only inside this directory, so run every command from here. 

```sh
cargo install cargo-fuzz
cargo run --example seeds     # writes seeds/<target>/, partly through the writers
cargo fuzz list
cargo fuzz run lp_reader corpus/lp_reader seeds/lp_reader -- -max_total_time=600
```
