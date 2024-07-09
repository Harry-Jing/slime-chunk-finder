//! Functions for working with slime chunks.

use crate::coords::ChunkCoords;
use crate::java_rand;

/// Returns true if the chunk at the given coordinates is a slime chunk.
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

/// Returns a list of chunk coordinates that form a circle with the given radius.
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

/// Returns the number of slime chunks in a ring with the given radius around the given chunk.
pub fn count_slime_chunks_in_ring(
    world_seed: i64,
    chunk_x: i32,
    chunk_z: i32,
    radius_long: i32,
    radius_short: i32,
) -> i32 {
    let lage_circle = get_circle_mask(radius_long);
    let small_circle = get_circle_mask(radius_short);

    let circle_mask_coords = lage_circle.iter().filter(|&u| !small_circle.contains(u)).collect::<Vec<_>>();
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
