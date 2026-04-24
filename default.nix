{
  rustPlatform,
  glib,
  pkg-config,
}:
rustPlatform.buildRustPackage {
  name = "nix-parser";
  src = ./.;
  cargoHash = "sha256-+XpN2sYdYmYPwL8GuH1/JbIOaNczR6rGMqd7LQB9UIE=";
  buildInputs = [
    glib
  ];
  nativeBuildInputs = [
    pkg-config
  ];
}
