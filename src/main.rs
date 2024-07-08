mod java_rand;
mod coords;

use coords::ChunkCoords;

fn is_slime_chunk(world_seed: i64, chunk_x: i32, chunk_z: i32) -> bool {
    let seed = world_seed
        + (chunk_x as i64 * chunk_x as i64) * 0x4C1906
        + (chunk_x as i64 * 0x5AC0DB)
        + (chunk_z as i64 * chunk_z as i64) * 0x4307A7
        + (chunk_z as i64 * 0x5F24F)
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

fn count_slime_chunks_in_radius(world_seed: i64, chunk_x: i32, chunk_z: i32, radius: i32) -> i32 {
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

fn main() {
    let world_seed = 7584197480721263469;
    let min_chunk_x = -1000;
    let max_chunk_x = 1000;
    let min_chunk_z = -1000;
    let max_chunk_z = 1000;

    let mut result: Vec<(ChunkCoords, i32)> = Vec::new();

    print!("Counting slime chunks in radius 8 for all chunks...");
    for x in min_chunk_x..=max_chunk_x {
        for z in min_chunk_z..=max_chunk_z {
            result.push((ChunkCoords::new(x, z), count_slime_chunks_in_radius(world_seed, x, z, 8)));
        }
    }
    println!("Finished counting slime chunks in radius 8 for all chunks. Sorting...");
    result.sort_by_key(|k| -k.1);
    println!("{:?}", result[0]);
}