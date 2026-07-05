{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs";

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};

    # Base Rust package
    rustPackage = pkgs.rustPlatform.buildRustPackage {
      pname = "stargem_server";
      version = "0.1.0";
      src = pkgs.lib.cleanSource ./.;
      cargoHash = pkgs.lib.fakeHash;
    };

    nativeDeps = with pkgs; [
      alsa-lib
      systemd
      libxkbcommon
      xorg.libX11
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
      wayland
      vulkan-loader
    ];
  in {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      pname = "far_limits";
      version = "0.1.0";
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
      buildInputs = nativeDeps;
    };

    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [ rustc cargo pkg-config ] ++ nativeDeps;

      shellHook = ''
        export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath nativeDeps}
      '';
    };
  };
}
