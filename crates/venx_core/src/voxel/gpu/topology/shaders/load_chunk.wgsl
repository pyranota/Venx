@group(0)
@binding(0)
var<storage, read_write> load_chunks: LoadChunkInfo;

@group(0)
@binding(1)
var<storage, read_write> affected_levels: array<u32>;

struct LoadChunkInfo {
    level_to_load: u32,
    positions: array<vec3<u32>>,
}
