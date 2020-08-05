use strum_macros::EnumIter;
use strum::IntoEnumIterator;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
pub enum ComponentType {
    Null,
    Position,
    Fighter,
    Inventory,
    Playable,
}

pub struct NullComponent {
    owner: GenerationalIndex
}

pub struct PositionComponent {
    owner: GenerationalIndex,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct GenerationalIndex {
    index: usize,
    generation: u32
}

impl GenerationalIndex {
    pub fn index(&self) -> usize { self.index }
}

#[derive(Debug, Clone)]
struct AllocatorEntry {
    is_live: bool,
    generation: u32,
}

#[derive(Debug)]
pub struct GenerationalIndexAllocator {
    max_index: usize,
    entries: Vec<AllocatorEntry>,
    free: Vec<usize>
}

impl GenerationalIndexAllocator {
    pub fn new(n: usize) -> Self {
        Self {
            max_index: n,
            entries: vec![AllocatorEntry{is_live: false, generation: 0}; n],
            free: (0..n).collect(),
        }
    }

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

struct ArrayEntry<T> {
    value: T,
    generation: u32
}

pub struct GenerationalIndexArray<T>(Vec<Option<ArrayEntry<T>>>);

impl<T> GenerationalIndexArray<T> {
    pub fn new(n: usize) -> Self {
        let mut a = Self(Vec::new());
        for i in 0..n {
            a.0.push(None);
        }
        a
    }

    pub fn set(&mut self, index: GenerationalIndex, value: T) {
        self.0[index.index()] = Some(ArrayEntry {value: value, generation: index.generation});
    }

    pub fn get(&self, index: GenerationalIndex) -> Option<&T> {
        match self.0.get(index.index()).unwrap() {
            Some(e) => {
                if e.generation == index.generation {
                    return Some(&e.value);
                }
                None
            },
            None => None,
        }
    }

    pub fn get_mut(&mut self, index: GenerationalIndex) -> Option<&mut T> {
        match self.0.get_mut(index.index()).unwrap() {
            Some(e) => {
                if e.generation == index.generation {
                    return Some(&mut e.value);
                }
                None
            },
            None => None,
        }
    }
}

type Entity = GenerationalIndex;
type EntityMap<T> = GenerationalIndexArray<T>;

pub struct State {
    entity_allocator: GenerationalIndexAllocator,
    
    null_components: EntityMap<NullComponent>,
    position_components: EntityMap<PositionComponent>,
}

