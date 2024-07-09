mod coords;
mod java_rand;
mod slime;

use coords::ChunkCoords;
use rayon::prelude::*;
use slime::count_slime_chunks_in_radius;
use std::fmt::Display;
use std::io;
use std::time::Instant;

use std::io::Write;

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

fn prompt_for_value<T: std::str::FromStr + Display>(prompt: &str, default: T) -> T {
    loop {
        print!("{} (default: {}): ", prompt, default);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input.is_empty() {
            return default;
        }

        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => {
                println!("Invalid input. Using default value: {}", default);
                return default;
            }
        }
    }
}

fn user_cli() {
    let world_seed = prompt_for_value("Enter world seed:", 0);
    let min_chunk_x = prompt_for_value("Enter minimum chunk x-coordinate:", -1000);
    let max_chunk_x = prompt_for_value("Enter maximum chunk x-coordinate", 1000);
    let min_chunk_z = prompt_for_value("Enter minimum chunk z-coordinate", -1000);
    let max_chunk_z = prompt_for_value("Enter maximum chunk z-coordinate", 1000);

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
        println!(
            "({}, {}): {} slime chunks",
            chunk_count.coords.x * 16,
            chunk_count.coords.z * 16,
            chunk_count.count
        );
    }

    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

fn main() {
    user_cli();
}
