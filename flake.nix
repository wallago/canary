{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
    claude-code = {
      url = "github:sadjow/claude-code-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    esp-rs-nix.url = "github:leighleighleigh/esp-rs-nix";
    nixpkgs-esp-dev.url = "github:mirrexagon/nixpkgs-esp-dev";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      naersk,
      claude-code,
      esp-rs-nix,
      nixpkgs-esp-dev,
      ...
    }:
    # ── System-agnostic outputs (modules) live out here ──
    {
      nixosModules.default = import ./nix/module.nix self;
      homeModules.default = import ./nix/hm-module.nix self;
    }
    # ── Then merge the per-system outputs onto it ──
    //
      flake-utils.lib.eachSystem
        [
          "x86_64-linux"
          "aarch64-linux"
        ]
        (
          system:
          let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs {
              inherit
                system
                overlays
                ;
              config.allowUnfree = true;
            };

            # ── Toolchain ─────────────────────────────────────────────
            rust = pkgs.rust-bin.nightly.latest.default;

            naersk' = pkgs.callPackage naersk {
              cargo = rust;
              rustc = rust;
            };

            # ── Build helper ──────────────────────────────────────────
            buildApp =
              { release }:
              let
                name = "canary";
                desc = "Rust firmware turning an ESP32-S3 touch LCD into an always-on desk widget for environment and status info.";
              in
              pkgs.callPackage ./nix/package.nix {
                inherit
                  naersk'
                  release
                  name
                  desc
                  ;
                src = ./.;
              };

            # ── Claude Settings ──────────────────────────────────────
            claude = claude-code.packages.${system}.default;

            # ── ESP ──────────────────────────────────────────────────
            esp-dev = nixpkgs-esp-dev.packages.${system}.esp-idf-full;
            esp-rs = esp-rs-nix.packages.${system}.esp-rs;

            # ── Tooling shared by the dev shell and CI ───────────────
            ciTools = with pkgs; [
              rust
              # rust tooling
              cargo-nextest
              cargo-edit

              cargo-audit
              cargo-machete
              cargo-deny
              typos
              committed
              git-cliff
              taplo
              editorconfig-checker

              # nix tooling
              nixfmt
              statix
              deadnix

              # ESP
              esp-dev
              esp-rs
              espflash
            ];
          in
          {
            # ── Packages ──────────────────────────────────────────────
            packages = rec {
              canary = buildApp { release = true; };
              canary-debug = buildApp { release = false; };
              default = canary;
            };

            # ── Checks (nix flake check) ─────────────────────────────
            checks.check = self.packages.${system}.canary-debug;

            # ── Dev Shell (nix develop) ──────────────────────────────
            devShells.default = pkgs.mkShell {
              buildInputs =
                ciTools
                ++ (with pkgs; [
                  rust-analyzer
                  just
                  claude
                  nodejs
                ]);
              shellHook = ''
                export PATH="${esp-rs}/bin:$PATH"
              '';
            };

            # ── CI Shell (nix develop .#ci) ──────────────────────────
            # Lean: just the toolchain + checks, no editor/claude/shellHook.
            devShells.ci = pkgs.mkShell {
              buildInputs = ciTools;
            };
          }
        );
}
