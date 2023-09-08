@group(4)
@binding(6)
var<storage, read_write> chunks_6: array<Chunk6>;

@group(4)
@binding(5)
var<storage, read_write> chunks_5: array<Chunk5>;

@group(4)
@binding(4)
var<storage, read_write> chunks_4: array<Chunk4>;

@group(4)
@binding(3)
var<storage, read_write> chunks_3: array<Chunk3>;

@group(4)
@binding(2)
var<storage, read_write> chunks_2: array<Chunk2>;

@group(4)
@binding(1)
var<storage, read_write> chunks_1: array<Chunk1>;

struct ChunkMeta {
    lod_offsets_strengh: u32,
    loaded: u32,
    position: vec3<u32>,
}

struct Chunk6 {
     metadata: ChunkMeta,
    data: array<u32, 262144>
}

struct Chunk5 {
     metadata: ChunkMeta,
    data: array<u32, 32768>
}

struct Chunk4 {
     metadata: ChunkMeta,
    data: array<u32, 4096>
}

struct Chunk3 {
     metadata: ChunkMeta,
    data: array<u32, 512>
}

struct Chunk2 {
     metadata: ChunkMeta,
    data: array<u32, 64>
}

struct Chunk1 {
     metadata: ChunkMeta,
    data: array<u32, 64>
}