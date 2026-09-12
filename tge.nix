{ lib, rustPlatform }:
let
  cargo_toml = lib.importTOML ./Cargo.toml;
in
rustPlatform.buildRustPackage rec {
  pname = "tge";
  version = cargo_toml.package.version;

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  doCheck = true;

  meta = {
    description = cargo_toml.package.description;
    homepage = cargo_toml.package.homepage;
    license = lib.licenses.gpl2Plus;
    maintainers = cargo_toml.package.authors;
    sourceProvenance = lib.sourceTypes.fromSource;
    mainProgram = pname;
  };
}
