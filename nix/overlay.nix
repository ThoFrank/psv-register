(final: prev: {
  wasm-bindgen-cli = prev.wasm-bindgen-cli.overrideAttrs (old: {
    version = "0.2.87";
    src = prev.fetchCrate {
      pname = "wasm-bindgen-cli";
      version = final.wasm-bindgen-cli.version;
      sha256 = "sha256-0u9bl+FkXEK2b54n7/l9JOCtKo+pb42GF9E1EnAUQa0=";
    };
    cargoDeps = old.cargoDeps.overrideAttrs (_: {
      src =  final.wasm-bindgen-cli.src; # You need to pass "src" here again,
                                         # otherwise the old "src" will be used.
      outputHash = "sha256-AsZBtE2qHJqQtuCt/wCAgOoxYMfvDh8IzBPAOkYSYko";
    });
  });
  trunk = prev.trunk.overrideAttrs (old: {
    version = "0.17.5";
    src = prev.fetchCrate {
      pname = "trunk";
      version = final.trunk.version;
      sha256 = "sha256-kOZmvpM13ccP1rAp4NFJigvhperlBTqOHYK/Y2p/nYk";
    };
    doCheck = false;
    cargoDeps = old.cargoDeps.overrideAttrs (_: {
      src =  final.trunk.src; # You need to pass "src" here again,
                              # otherwise the old "src" will be used.
      outputHash = "sha256-CPwestYkqG6TaFsM99GVcbfGJNbdO9IphazZQwmt93g";
    });
  });
})

