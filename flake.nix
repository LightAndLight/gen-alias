{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    cargo2nix = {
      url = "github:cargo2nix/cargo2nix";
      # inputs.rust-overlay.follows = "rust-overlay";
    };
  };
  outputs = { self, nixpkgs, flake-utils, rust-overlay, cargo2nix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            rust-overlay.overlays.default
            cargo2nix.overlays.cargo2nix
          ];
        };

        rustVersion = "1.78.0";

      in {
        devShell =
          pkgs.mkShell {
            buildInputs = [
              (pkgs.rust-bin.stable.${rustVersion}.default.override {
                extensions = [
                  "cargo"
                  "clippy"
                  "rustc"
                  "rust-src"
                  "rustfmt"
                  "rust-analyzer"
                ];
              })
              cargo2nix.packages.${system}.cargo2nix

              pkgs.pkg-config
              pkgs.glib
              pkgs.cairo
              pkgs.gtk4
            ];
          };

        packages =
          let
            rustPkgs =
              pkgs.rustBuilder.makePackageSet {
                inherit rustVersion;
                packageFun = import ./Cargo.nix;
                packageOverrides = pkgs: with pkgs.rustBuilder.rustLib; pkgs.rustBuilder.overrides.all ++ [
                  (makeOverride {
                    name = "glib-sys";
                    overrideAttrs = drv: {
                      buildInputs = (drv.buildInputs or []) ++ [
                        pkgs.pkg-config
                        pkgs.glib
                      ];
                    };
                  })
                  
                  (makeOverride {
                    name = "cairo-sys-rs";
                    overrideAttrs = drv: {
                      buildInputs = (drv.buildInputs or []) ++ [
                        pkgs.pkg-config
                        pkgs.cairo
                      ];
                    };
                  })
                  
                  (makeOverride {
                    name = "gobject-sys";
                    overrideAttrs = drv: {
                      buildInputs = (drv.buildInputs or []) ++ [
                        pkgs.pkg-config
                        pkgs.glib
                      ];
                    };
                  })
                  
                  (makeOverride {
                    name = "graphene-sys";
                    overrideAttrs = drv: {
                      buildInputs = (drv.buildInputs or []) ++ [
                        pkgs.pkg-config
                        pkgs.glib
                        pkgs.graphene
                      ];
                    };
                  })
                  
                  (makeOverride {
                    name = "gio-sys";
                    overrideAttrs = drv: {
                      buildInputs = (drv.buildInputs or []) ++ [
                        pkgs.pkg-config
                        pkgs.glib
                      ];
                    };
                  })
                  
                  (makeOverride {
                    name = "pango-sys";
                    overrideAttrs = drv: {
                      buildInputs = (drv.buildInputs or []) ++ [
                        pkgs.pkg-config
                        pkgs.pango
                      ];
                    };
                  })
                  
                  (makeOverride {
                    name = "gdk-pixbuf-sys";
                    overrideAttrs = drv: {
                      buildInputs = (drv.buildInputs or []) ++ [
                        pkgs.pkg-config
                        pkgs.gdk-pixbuf
                      ];
                    };
                  })
                  
                  (makeOverride {
                    name = "gdk4-sys";
                    overrideAttrs = drv: {
                      buildInputs = (drv.buildInputs or []) ++ [
                        pkgs.pkg-config
                        pkgs.gtk4
                      ];
                    };
                  })
                  
                  (makeOverride {
                    name = "gsk4-sys";
                    overrideAttrs = drv: {
                      buildInputs = (drv.buildInputs or []) ++ [
                        pkgs.pkg-config
                        pkgs.gtk4
                      ];
                    };
                  })
                  
                  (makeOverride {
                    name = "gtk4-sys";
                    overrideAttrs = drv: {
                      buildInputs = (drv.buildInputs or []) ++ [
                        pkgs.pkg-config
                        pkgs.gtk4
                      ];
                    };
                  })
                ];
              };
          in rec {
            gen-alias = rustPkgs.workspace.gen-alias {};
            default = gen-alias;
          };
      }
    );
}
