{ rustPlatform, lib, ... }:

rustPlatform.buildRustPackage {
  pname = "libcustom_command";
  version = "1.0";

  src = ./.;

  cargoHash = "sha256-HIMTRNfxPI9vLkmNneXXSuMT3r8+B+75P1Mhi7ygZq4=";

  meta = with lib; {
    description = "Anyrun custom command library";
    license = licenses.gpl2;
    platforms = platforms.all;
  };
}
