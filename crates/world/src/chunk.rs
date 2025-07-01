use crate::palette::{PaletteType, PalettedContainer};

pub const SECTION_WIDTH: usize = 16;
pub const SECTION_HEIGHT: usize = 16;
pub const SECTION_VOLUME: usize = SECTION_WIDTH * SECTION_WIDTH * SECTION_HEIGHT;

pub const BIOME_WIDTH: usize = 4;
pub const BIOME_HEIGHT: usize = 4;
pub const BIOME_VOLUME: usize = BIOME_WIDTH * BIOME_WIDTH * BIOME_HEIGHT;

pub struct ChunkSection {
    block_count: u16,
    block_states: PalettedContainer,
    biomes: PalettedContainer,
}

impl ChunkSection {
    pub fn new() -> Self {
        Self {
            block_count: 0,
            block_states: PalettedContainer::new(PaletteType::Block, SECTION_VOLUME),
            biomes: PalettedContainer::new(PaletteType::Biome, BIOME_VOLUME),
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> u32 {
        self.block_states.get(y * SECTION_WIDTH * SECTION_HEIGHT + z * SECTION_WIDTH + x)
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: u32) {
        self.block_states.set(y * SECTION_WIDTH * SECTION_HEIGHT + z * SECTION_WIDTH + x, block);
    }
}

pub struct ChunkColumn {
    sections: Vec<ChunkSection>,
}

impl ChunkColumn {
    pub fn new() -> Self {
        let mut sections = Vec::with_capacity(24);
        for _ in 0..24 {
            sections.push(ChunkSection::new());
        }

        Self { sections }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> u32 {
        let section = &self.sections[y / SECTION_HEIGHT];
        section.get_block(x, y % SECTION_HEIGHT, z)
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: u32) {
        let section = &mut self.sections[y / SECTION_HEIGHT];
        section.set_block(x, y % SECTION_HEIGHT, z, block);
    }
}