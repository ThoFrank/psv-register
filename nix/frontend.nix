{ pkgs }:
let
  rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-wasm-toolchain.toml;

in
pkgs.rustPlatform.buildRustPackage {
  name = "psv-register-frontend";
  src = ./..;
  cargoLock.lockFile = ../Cargo.lock;
  nativeBuildInputs = [
    rust
    pkgs.trunk
    pkgs.wasm-bindgen-cli
  ];

  TRUNK_TOOLS_WASM_BINDGEN = pkgs.wasm-bindgen-cli.version;

  buildPhase = ''
    mkdir cache
    export XDG_CACHE_HOME=$(pwd)/cache
    cd frontend

    trunk build --release

    mkdir -p $out;
    cp -r dist $out/web
  '';

  installPhase = ''
    echo 'Skipping installPhase'
  '';
}

