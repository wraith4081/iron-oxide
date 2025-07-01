use std::mem::size_of;

const BITS_PER_U64: usize = size_of::<u64>() * 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteType {
    Block,
    Biome,
}

impl PaletteType {
    fn min_bits(&self) -> u8 {
        match self {
            PaletteType::Block => 4,
            PaletteType::Biome => 1,
        }
    }

    fn max_bits(&self) -> u8 {
        match self {
            PaletteType::Block => 8,
            PaletteType::Biome => 3,
        }
    }

    fn direct_bits(&self) -> u8 {
        match self {
            PaletteType::Block => 15,
            PaletteType::Biome => 6,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Palette {
    Single(u32),
    Indirect(Vec<u32>),
    Direct,
}

#[derive(Debug, Clone)]
pub struct PalettedContainer {
    palette: Palette,
    storage: Vec<u64>,
    bits_per_entry: u8,
    palette_type: PaletteType,
    size: usize,
}

impl PalettedContainer {
    pub fn new(palette_type: PaletteType, size: usize) -> Self {
        Self {
            palette: Palette::Single(0),
            storage: Vec::new(),
            bits_per_entry: 0,
            palette_type,
            size,
        }
    }

    pub fn get(&self, index: usize) -> u32 {
        if self.bits_per_entry == 0 {
            return match self.palette {
                Palette::Single(id) => id,
                _ => unreachable!(),
            };
        }

        let values_per_u64 = BITS_PER_U64 / self.bits_per_entry as usize;
        let u64_index = index / values_per_u64;
        let sub_index = index % values_per_u64;
        let mask = (1 << self.bits_per_entry) - 1;

        let value = (self.storage[u64_index] >> (sub_index * self.bits_per_entry as usize)) & mask;

        match &self.palette {
            Palette::Indirect(palette) => palette[value as usize],
            Palette::Direct => value as u32,
            Palette::Single(_) => unreachable!(),
        }
    }

    pub fn set(&mut self, index: usize, value: u32) {
        // For now, we only support direct palette for simplicity
        self.bits_per_entry = self.palette_type.direct_bits();
        self.palette = Palette::Direct;

        let values_per_u64 = BITS_PER_U64 / self.bits_per_entry as usize;
        let u64_count = (self.size + values_per_u64 - 1) / values_per_u64;
        if self.storage.is_empty() {
            self.storage.resize(u64_count, 0);
        }

        let u64_index = index / values_per_u64;
        let sub_index = index % values_per_u64;
        let mask = (1 << self.bits_per_entry) - 1;

        let mut u64_value = self.storage[u64_index];
        u64_value &= !(mask << (sub_index * self.bits_per_entry as usize));
        u64_value |= (value as u64 & mask) << (sub_index * self.bits_per_entry as usize);
        self.storage[u64_index] = u64_value;
    }
}