{pkgs}:
pkgs.rustPlatform.buildRustPackage {
  name = "psv-reigister";
  src = ./.;
  srcRoot = ./backend;
  cargoLock.lockFile = ./Cargo.lock;
  cargoSha256 = pkgs.lib.fakeSha256;
}
