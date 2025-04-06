{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      fenix,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        # https://github.com/nix-community/fenix/issues/178
        cargo = fenix.packages.${system}.stable.cargo.overrideAttrs (old: {
          postBuild = pkgs.lib.optionalString pkgs.stdenv.isDarwin ''
            cargo="./cargo/bin/cargo"
            install_name_tool \
              -change "/usr/lib/libcurl.4.dylib" "${pkgs.curl.out}/lib/libcurl.4.dylib" \
              "$cargo"
          '';
        });
        rustToolchain =
          with fenix.packages.${system};
          combine [
            cargo
            stable.rustc
            stable.clippy
            stable.rustfmt
          ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            rust-analyzer
          ];
        };
      }
    );
}
