#[derive(Debug)]
pub struct ChunkCoords {
    pub x: i32,
    pub z: i32,
}

impl ChunkCoords {
    pub fn new(x: i32, z: i32) -> Self {
        ChunkCoords { x, z }
    }
}
