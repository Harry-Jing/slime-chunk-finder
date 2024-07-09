//! Coordinates for the game world.

/// Coordinates for a chunk.
#[derive(PartialEq)]
pub struct ChunkCoords {
    pub x: i32,
    pub z: i32,
}

impl ChunkCoords {
    /// Creates a new `ChunkCoords` with the given x and z coordinates.
    pub fn new(x: i32, z: i32) -> Self {
        ChunkCoords { x, z }
    }
}
