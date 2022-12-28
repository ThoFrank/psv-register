{pkgs}:
pkgs.rustPlatform.buildRustPackage {
  name = "psv-reigister";
  src = ./frontend;
  srcRoot = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  cargoSha256 = "sha256-JmBZcDVYJaK1cK05cxx5BrnGWp4t8ca6FLUbvIot67s=";
}

