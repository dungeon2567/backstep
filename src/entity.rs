use crate::storage::{Chunk, Page};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Entity(u64);

impl Entity {
    const INDEX_BITS: u32 = 21;
    const GENERATION_BITS: u32 = 43;
    const INDEX_MASK: u64 = (1u64 << Self::INDEX_BITS) - 1;
    const GENERATION_MASK: u64 = (1u64 << Self::GENERATION_BITS) - 1;

    #[inline(always)]
    /// Create a new Entity from index and generation
    /// Generation is wrapped to 43 bits (lowest 43 bits are used)
    pub fn new(index: u32, generation: u64) -> Self {
        let index = (index as u64) & Self::INDEX_MASK;
        let generation = generation & Self::GENERATION_MASK; // Wrap to 43 bits

        Entity((index << Self::GENERATION_BITS) | generation)
    }

    #[inline(always)]
    pub fn none() -> Self {
        Entity(0)
    }

    #[inline(always)]
    pub fn is_none(&self) -> bool {
        self.generation() == 0
    }

    #[inline(always)]
    pub fn index(&self) -> u32 {
        ((self.0 >> Self::GENERATION_BITS) & Self::INDEX_MASK) as u32
    }

    #[inline(always)]
    pub fn set_index(&mut self, index: u32) {
        let index = (index as u64) & Self::INDEX_MASK;
        self.0 = (self.0 & Self::GENERATION_MASK) | (index << Self::GENERATION_BITS);
    }

    #[inline(always)]
    /// Get the generation (lowest 43 bits)
    pub fn generation(&self) -> u64 {
        self.0 & Self::GENERATION_MASK
    }

    #[inline(always)]
    /// Set the generation (lowest 43 bits)
    pub fn set_generation(&mut self, generation: u64) {
        let generation = generation & Self::GENERATION_MASK;
        self.0 = (self.0 & !Self::GENERATION_MASK) | generation;
    }
}

impl std::fmt::Debug for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Entity(id = {}, generation = {})",
            self.index(),
            self.generation()
        )
    }
}

#[allow(non_snake_case)]
mod __backstep_component_Entity {
    use super::Entity;
    use crate::storage::{Chunk, Page};

    pub(super) static mut ID: u32 = 0;

    pub(super) static DEFAULT_CHUNK: Chunk<Entity> = Chunk {
        presence_mask: 0,
        fullness_mask: 0,
        changed_mask: 0,
        data: unsafe { std::mem::MaybeUninit::<[std::mem::MaybeUninit<Entity>; 64]>::uninit().assume_init() },
    };

    pub(super) static DEFAULT_PAGE: Page<Entity> = Page {
        presence_mask: 0,
        fullness_mask: 0,
        changed_mask: 0,
        count: 0,
        data: [&DEFAULT_CHUNK as *const Chunk<Entity> as *mut Chunk<Entity>; 64],
    };
}

impl crate::component::Component for Entity {
    fn id() -> u32 {
        unsafe { __backstep_component_Entity::ID }
    }

    fn get_default_chunk() -> *const Chunk<Self> {
        &__backstep_component_Entity::DEFAULT_CHUNK
    }

    fn get_default_page() -> *const Page<Self> {
        &__backstep_component_Entity::DEFAULT_PAGE
    }

    fn initialize(id: u32) {
        let _ = id;
    }
    fn schedule_cleanup_system(world: &mut crate::world::World) {
        let sys = crate::system::ComponentCleanupSystem::<Entity>::new(world);
        world.scheduler_mut().add_system(sys);
    }
}

