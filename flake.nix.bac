{
  description = "A flake for Venx voxel engine";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = {  self, nixpkgs, flake-utils }: 

    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in
        {
          devShells.default = import ./shell.nix { inherit pkgs; };


          # apps.bevy =
          #   let
          #     version = "1.5.2";
          #     inherit (pkgs) stdenv lib;
          #   in
          #   stdenv.mkDerivation
          #     {
          #       name = "bevy";
          #       src = self;
          #       buildInputs = [ pkgs.stdenv.cc.cc.lib ];
          #       nativeBuildInputs = [ pkgs.stdenv.cc.cc.lib ];
          #       phases = [ "installPhase" ];
          #       # unpackPhase = ''
          #       #   mkdir -p $out/bin
          #       #   tar -xzf $src -C $out/bin
          #       # '';

          #       # this phase is not necessary, but it's here to show how to install
          #       installPhase = ''
          #       '';

          #       # meta = with nixpkgs.lib; {
          #       #   homepage = "https://github.com/GoogleCloudPlatform/cloud-spanner-emulator";
          #       #   description =
          #       #     "Cloud Spanner Emulator is a local emulator for the Google Cloud Spanner database service.";
          #       #   platforms = platforms.linux;
          #       # };
          #     };
          

          apps.demo = {
            type = "app";
            buildPhase = "cargo build";
            program = 
              
          };
        }
      );

    # packages.x86_64-linux.hello = nixpkgs.legacyPackages.x86_64-linux.hello;

    # packages.x86_64-linux.default = self.packages.x86_64-linux.hello;

    # # Executed by `nix run .#<name>`
    # apps.x86_64-linux.demo = {
    #   type = "app";
    #   program = "<store-path>";
    # };

    # devShells."<system>"."<name>" = derivation;
    # # Used by `nix develop`
    # devShells."<system>".default = derivation;
    # # Hydra build jobs
    # hydraJobs."<attr>"."<system>" = derivation;

  
}
