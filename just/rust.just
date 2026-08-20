# Flash the firmware to a connected board and open the monitor
[group('dev')]
run *ARGS:
    cd firmware && cargo run {{ ARGS }}

# Build the optimized release firmware
[group('dev')]
build:
    cd firmware && cargo build --release

# Type-check both crates (fast feedback, no binary)
[group('dev')]
check:
    cargo check -p canary-core --all-targets
    cd firmware && cargo check --all-targets

# Format Rust + Nix sources in place
[group('dev')]
fmt:
    cargo fmt --all
    nixfmt flake.nix package.nix module.nix

# Run the host test suite. Usage: just test [pattern]
[group('dev')]
test *ARGS:
    cargo nextest run -p canary-core {{ ARGS }}
