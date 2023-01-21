{pkgs, frontend}:
pkgs.rustPlatform.buildRustPackage {
  buildInputs = [
    pkgs.makeWrapper
    pkgs.sqlite.dev
  ] ++ pkgs.lib.lists.optional pkgs.stdenv.isDarwin [pkgs.darwin.apple_sdk.frameworks.Security];
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
