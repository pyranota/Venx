{
  description = "A flake for Venx voxel engine";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = {  self, nixpkgs, flake-utils }: 

    flake-utils.lib.eachDefaultSystem
      (system:
        let pkgs = nixpkgs.legacyPackages.${system}; in
        {
          devShells.default = import ./shell.nix { inherit pkgs; };

          apps.demo = {
            type = "app";
            program = "<store-path>";
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
