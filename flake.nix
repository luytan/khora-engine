{
  description = "khora-engine devshell";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    fenix.url = "github:nix-community/fenix";
    git-hooks.url = "github:cachix/git-hooks.nix";
  };
  outputs =
    {
      self,
      nixpkgs,
      fenix,
      git-hooks,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = fn: nixpkgs.lib.genAttrs supportedSystems (system: fn system);
      pkgs = system: nixpkgs.legacyPackages.${system};
      fenixpkgs = system: fenix.packages.${system};
      toolchainFor =
        system:
        (fenixpkgs system).combine [
          (fenixpkgs system).stable.cargo
          (fenixpkgs system).stable.rustc
          (fenixpkgs system).latest.rustfmt
          (fenixpkgs system).stable.clippy
          (fenixpkgs system).stable.rust-src
        ];
    in
    {
      devShells = forAllSystems (system: {
        default = (pkgs system).mkShell {
          packages = [
            (toolchainFor system)
          ];
          RUST_SRC_PATH = "${(fenixpkgs system).stable.rust-src}/lib/rustlib/src/rust/library";
          RUST_BACKTRACE = "1";
        };
      });
    };
}
