{ stdenv, fetchFromGitHub }: stdenv.mkDerivation rec {
  pname = "fastchess";
  version = "31-MAR-2025";

  src = fetchFromGitHub {
    owner = "Disservin";
    repo = "fastchess";
    rev = "57a7b4e7bce27fc96c16125eb1d480fb57091f0d";
    sha256 = "sha256-tqdnMpohOgniIsugEs678WJN+W2qQWyG4+ndE01NEF0=";
  };
  meta.mainProgram = "fastchess";

  enableParallelBuilding = true;

  installPhase = ''
    mkdir -p $out/bin
    cp fastchess $out/bin
  '';
}
