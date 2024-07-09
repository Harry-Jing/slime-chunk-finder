use crate::coords::ChunkCoords;
use crate::java_rand;

fn is_slime_chunk(world_seed: i64, chunk_x: i32, chunk_z: i32) -> bool {
    let seed = (world_seed
        + (chunk_x as i64 * chunk_x as i64) * 0x4C1906
        + (chunk_x as i64 * 0x5AC0DB)
        + (chunk_z as i64 * chunk_z as i64) * 0x4307A7
        + (chunk_z as i64 * 0x5F24F))
        ^ 0x3AD8025F;
    let rng = java_rand::Random::new(seed as u64);
    rng.next_int_bound(10) == 0
}

fn get_circle_mask(radius: i32) -> Vec<ChunkCoords> {
    let mut coords = Vec::new();
    for x in -radius..=radius {
        for z in -radius..=radius {
            if x * x + z * z <= radius * radius {
                coords.push(ChunkCoords::new(x, z));
            }
        }
    }
    coords
}

pub fn count_slime_chunks_in_radius(
    world_seed: i64,
    chunk_x: i32,
    chunk_z: i32,
    radius: i32,
) -> i32 {
    let circle_mask_coords = get_circle_mask(radius);
    let mut count = 0;
    for coord in circle_mask_coords {
        let x = chunk_x + coord.x;
        let z = chunk_z + coord.z;
        if is_slime_chunk(world_seed, x, z) {
            count += 1;
        }
    }
    count
}
