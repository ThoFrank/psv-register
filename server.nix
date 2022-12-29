{pkgs, frontend}:
pkgs.rustPlatform.buildRustPackage {
  buildInputs = [
    pkgs.makeWrapper
  ];
  name = "psv-register";
  src = ./.;
  srcRoot = ./backend;
  cargoLock.lockFile = ./Cargo.lock;
  cargoSha256 = pkgs.lib.fakeSha256;
  postFixup = ''
    wrapProgram $out/bin/backend \
      --set WEBPAGE ${frontend}/web
    ln $out/bin/backend $out/bin/psv-register
  '';
}
