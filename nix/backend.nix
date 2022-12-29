{pkgs, frontend}:
pkgs.rustPlatform.buildRustPackage {
  buildInputs = [
    pkgs.makeWrapper
  ];
  name = "psv-register";
  src = ./..;
  srcRoot = ../backend;
  cargoLock.lockFile = ../Cargo.lock;
  postFixup = ''
    wrapProgram $out/bin/backend \
      --set WEBPAGE ${frontend}/web
    ln $out/bin/backend $out/bin/psv-register
  '';
}
