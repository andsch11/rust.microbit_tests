# Status
- Tested with micro:bit V2.21
- LED blinking
- Can Run/Debug from VSCode
- "cargo run" in command line prints RTT logs to Desktop Console

# What is needed to get started
VSCode Extensions
  - rust-analyzer
  - Maybe "Debugger for probe-rs"
  - Maybe "CodeLLDB"

rustup target add thumbv7em-none-eabihf
cargo install cargo-binstall
cargo binstall probe-rs-tools

# Troubleshooting
List attached debug probes
> probe-rs list 

The numbers from the Probe are needed in config.toml for --probe
> runner = "probe-rs run --chip nRF52833_xxAA --probe 0d28:0204"