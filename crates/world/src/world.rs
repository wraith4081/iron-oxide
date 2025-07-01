use crate::chunk::ChunkColumn;
use std::collections::HashMap;

pub struct World {
    chunks: HashMap<(i32, i32), ChunkColumn>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn get_chunk(&mut self, x: i32, z: i32) -> &mut ChunkColumn {
        if !self.chunks.contains_key(&(x, z)) {
            let chunk = self.generate_chunk(x, z);
            self.chunks.insert((x, z), chunk);
        }
        self.chunks.get_mut(&(x, z)).unwrap()
    }

    fn generate_chunk(&self, _chunk_x: i32, _chunk_z: i32) -> ChunkColumn {
        let mut chunk = ChunkColumn::new();

        for x in 0..16 {
            for z in 0..16 {
                chunk.set_block(x, 0, z, 1); // Stone
                chunk.set_block(x, 1, z, 2); // Dirt
                chunk.set_block(x, 2, z, 2); // Dirt
                chunk.set_block(x, 3, z, 2); // Dirt
                chunk.set_block(x, 4, z, 3); // Grass
            }
        }

        chunk
    }
}