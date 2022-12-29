{pkgs}:
let
  rust = pkgs.rust-bin.fromRustupToolchainFile ./rust-wasm-toolchain.toml;
  
in pkgs.rustPlatform.buildRustPackage{
  name = "psv-register-frontend";
  src = ./.;
  cargoSha256 = "sha256-OimJZKqwjJrA/0Mw/EYclbF5xFUcxNtcVH/54I0OEeM=";
  nativeBuildInputs = [
    rust
    pkgs.trunk
    pkgs.wasm-bindgen-cli
  ];

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

