# default.nix
with import <nixpkgs> {};

stdenv.mkDerivation {
    name = "dev-environment"; # Probably put a more meaningful name here
    buildInputs = [
        #Backend
        #From nixos.wiki/wiki/Rust
        rustc
        cargo
        gcc
        rustfmt
        clippy

        #Needed to build non-rust packages (cargo build complained, these two packages fixed it)
        pkg-config
        openssl
    ];

    # Certain Rust tools won't work without this
    # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
    # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
    # RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}