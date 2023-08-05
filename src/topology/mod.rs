mod cache;
mod graph;
mod node;
mod traverse;
/**

Topology is a Directed Acyclic Graph which contains block placement including: Solid, Non-solid and Transparent Voxels.
ZERO coordinates starts from bottom, left, front side and can be only positive.
World size can only be the power of 2. For example: 16,32,1024,2048...

It can:
    1 - Be filled with voxels.
    2 - Merged (symmetric) to reduce size.
    3 - Edited.
    4 - Solidified


1. Filling with voxels
    It takes 3d Matrix as input.
    Checks for already existing parts in the graph.
    If yes: Adds; no: Creates new branch;
2. Merging:
    After Merging, Filling chunks with non-air voxels become inactive.
    So... You have to make sure you filled averything before merging it.
    Instead of merging existing graph, it creates new and filles it with new elements.


3. Editing:
    Editing...

4. Solidifying:
    Looks up on voxel attributes and its positions and writes down if node consists of fully solid blocks
*/

fn placeholder() {}
