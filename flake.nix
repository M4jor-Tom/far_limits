{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs";

  outputs = { self, nixpkgs }:
  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
      pname = "far_limits";
      version = "0.1.0";
      src = ./.;
      cargoLock.lockFile = ./Cargo.lock;
      buildInputs = with pkgs; [
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
    };

    devShells.${system}.default = pkgs.mkShell {
      packages = with pkgs; [
        rustc
        cargo

        pkg-config

        # Audio
        alsa-lib

        # Devices
        systemd

        # Keyboard / input (X11 + Wayland)
        libxkbcommon

        # X11
        xorg.libX11
        xorg.libXcursor
        xorg.libXrandr
        xorg.libXi

        # Wayland (safe to keep even on X11)
        wayland

        # GPU
        vulkan-loader
      ];

      shellHook = ''
        export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
          pkgs.libxkbcommon
          pkgs.xorg.libX11
          pkgs.vulkan-loader
          pkgs.alsa-lib
        ]}
      '';
    };
  };
}
