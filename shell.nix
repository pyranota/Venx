{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell rec {
  stdenv = pkgs.clangStdenv;
  buildInputs = with pkgs; [
    libGL
    alsa-lib
    openssl
    clang
    freetype
    cmake
    udev
    vulkan-loader
    vulkan-headers
    xorg.libX11
    libxkbcommon
    fontconfig
    wayland
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    xorg.libxcb
  ];
  nativeBuildInputs = with pkgs; [ pkg-config ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
  WINIT_UNIX_BACKEND = "x11";
}
# NIXPKGS_ALLOW_UNFREE=1 nix run --override-input nixpkgs nixpkgs/nixos-23.05 --impure github:guibou/nixGL#nixVulkanNvidia -- cargo r --bin bevy