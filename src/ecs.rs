use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum ComponentType {
    Null,
    Position,
    Fighter,
    Inventory,
    Playable,
}

#[derive(Eq, PartialEq)]
pub struct GenerationalIndex {
    index: usize,
    generation: u32
}

impl GenerationalIndex {
    pub fn index(&self) -> usize { self.index }
}

struct AllocatorEntry {
    is_live: bool,
    generation: u32,
}

pub struct GenerationalIndexAllocator {
    entries: Vec<AllocatorEntry>,
    free: Vec<usize>
}

impl GenerationalIndexAllocator {
    pub fn allocate(&mut self) -> GenerationalIndex {
        assert!(self.free.len() > 0, "No free generational indices available.");

        let id = self.free[0];
        let gen = self.entries[id].generation;
        self.free.drain(0..1);

        self.entries[id].generation += 1;
        self.entries[id].is_live = true;

        GenerationalIndex {
            index: id,
            generation: gen
        }

    }

    // TODO: should really return result instead
    pub fn deallocate(&mut self, index: GenerationalIndex) -> bool {
        if !self.entries[index.index()].is_live { 
            return false; // Already deallocated
        }
        self.entries[index.index()].is_live = false;
        self.free.push(index.index());
        true
    }

}
