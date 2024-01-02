<img src="assets/venx-splash.png" alt="Venx" />

## Introduction
Venx is feature rich highly performant voxel engine focused on large worlds and large rendering distances. It is hybrid of DaG based approach in creation of voxel engines and classical with 3d grid, ending up taking the best from both worlds.

> WIP Warning!
## Features
- [ ] Generating entire world during creation
- [ ] Adaptive LODs
- [ ] GPU accelerated loading and mesh creating
- [ ] GPU accelerated world creation
- [ ] Chunk culling
- [ ] Plats
- [ ] Modding friendly
- [ ] Rich Block type system
- [ ] Easy to use
- [ ] Multiplayer friendly
- [ ] Shadowing
- [ ] FVGen (Generator)
- [ ] Greedy meshing
- [ ] Importer
- [ ] AI path finder
## Getting started
### Cargo
### Nix/NixOS
Currently you can only run mca converter example 
`NIXPKGS_ALLOW_UNFREE=1 nix run --override-input nixpkgs nixpkgs/nixos-23.05 --impure github:guibou/nixGL#nixVulkanNvidia -- cargo r --release --package bevy_venx --bin bevy`

### Docker
`todo`