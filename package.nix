{
  rustPlatform,
  pkg-config,
  gexiv2_0_10,
  glib,
}:
rustPlatform.buildRustPackage {
  pname = "media-uploader";
  version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;
  src = ./.;
  cargoLock = ./Cargo.lock;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [
    # Plain `gexiv2` was removed in favor of explicit ABI-versioned
    # attributes; 0_10 is the long-standing GExiv2-0.10 ABI `gexiv2` used
    # to resolve to (0.14.x) - 0_16 is a newer, API-breaking rewrite.
    gexiv2_0_10
    glib
  ];
}
