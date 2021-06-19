let
  pkgs = import <nixpkgs> { };
  customBuildRustCrateForPkgs = pkgs: pkgs.buildRustCrate.override {
    defaultCrateOverrides = pkgs.defaultCrateOverrides // {
      "hello-cargo" = attrs: {
        buildInputs = [ pkgs.libiconv ];
      };
    };
  };
  generatedBuild = import ./Cargo.nix {
    inherit pkgs;
    buildRustCrateForPkgs = customBuildRustCrateForPkgs;
  };
in
generatedBuild.rootCrate.build
