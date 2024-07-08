mod coords;
mod java_rand;

use coords::ChunkCoords;
use rayon::prelude::*;
use std::io;
use std::io::Write;
use std::time::Instant;

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

#[derive(Debug)]
struct ChunkCount {
    pub coords: ChunkCoords,
    pub count: i32,
}

fn find_chunks_with_most_slime_chunks(
    min_chunk_x: i32,
    max_chunk_x: i32,
    min_chunk_z: i32,
    max_chunk_z: i32,
    world_seed: i64,
) -> Vec<ChunkCount> {
    println!("Counting slime chunks in radius 8 for all chunks...");
    let start_time = Instant::now();
    let coords: Vec<(i32, i32)> = (min_chunk_x..=max_chunk_x)
        .flat_map(|x| (min_chunk_z..=max_chunk_z).map(move |z| (x, z)))
        .collect();

    let result: Vec<ChunkCount> = coords
        .par_iter()
        .map(|&(x, z)| {
            let count = count_slime_chunks_in_radius(world_seed, x, z, 8);
            ChunkCount {
                coords: ChunkCoords::new(x, z),
                count,
            }
        })
        .collect();

    let elapsed_time = start_time.elapsed();
    println!(
        "Finished counting slime chunks in radius 8 for all chunks in {:?}. Sorting...",
        elapsed_time
    );
    let mut sorted_results = result;
    sorted_results.par_sort_by_key(|k| -k.count);
    sorted_results
}

fn prompt_for_value<T: std::str::FromStr>(prompt: &str, default: T) -> T {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse::<T>().unwrap_or(default)
}

fn main() {
    let world_seed = prompt_for_value("Enter world seed (default: 7584197480721263469):", 0);
    let min_chunk_x = prompt_for_value(
        "Enter minimum chunk x-coordinate (default: -10000):",
        -10000,
    );
    let max_chunk_x = prompt_for_value("Enter maximum chunk x-coordinate (default: 10000):", 10000);
    let min_chunk_z = prompt_for_value(
        "Enter minimum chunk z-coordinate (default: -10000):",
        -10000,
    );
    let max_chunk_z = prompt_for_value("Enter maximum chunk z-coordinate (default: 10000):", 10000);

    let sorted_results = find_chunks_with_most_slime_chunks(
        min_chunk_x,
        max_chunk_x,
        min_chunk_z,
        max_chunk_z,
        world_seed,
    );

    println!("Top 10 chunks with the most slime chunks in radius 8:");
    for i in 0..10 {
        let chunk_count = &sorted_results[i];
        println!("{:?} - {}", chunk_count.coords, chunk_count.count);
    }

    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}
