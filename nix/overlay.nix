(final: prev: {
  wasm-bindgen-cli = prev.wasm-bindgen-cli.overrideAttrs (old: {
    version = "0.2.83";
    src = prev.fetchCrate {
      pname = "wasm-bindgen-cli";
      version = final.wasm-bindgen-cli.version;
      sha256 = "sha256-+PWxeRL5MkIfJtfN3/DjaDlqRgBgWZMa6dBt1Q+lpd0=";
    };
    cargoDeps = old.cargoDeps.overrideAttrs (_: {
      src =  final.wasm-bindgen-cli.src; # You need to pass "src" here again,
                                         # otherwise the old "src" will be used.
      outputHash = "sha256-A+25E3yceMRvgiQyeGdjYSk+4VFLDDx7MSdNaA28TYA=";
    });
  });
})

