use crate::world::World;
use crate::storage::{Chunk, Page};
use backstep::system::TemporaryComponentCleanupSystem;
use std::alloc::Allocator;

pub trait Component
where
    Self: Sized + Clone + 'static,
{
    fn id() -> u32;

    fn get_default_chunk() -> *const Chunk<Self>;
    fn get_default_page() -> *const Page<Self>;

    fn initialize(_id: u32) {}

    fn schedule_cleanup_system(world: &mut World);

    fn clone_in(&self, _allocator: &dyn Allocator) -> Self {
        self.clone()
    }
}

#[derive(Clone, Copy)]
pub struct Destroyed();

// #[allow(non_upper_case_globals)]
// static mut __backstep_COMPONENT_ID_Destroyed: u32 = 1;

impl Destroyed {}

#[allow(non_snake_case)]
mod __backstep_component_Destroyed {
    use super::Destroyed;
    use crate::storage::{Chunk, Page};

    pub(super) static mut ID: u32 = 1;

    pub(super) static DEFAULT_CHUNK: Chunk<Destroyed> = Chunk {
        presence_mask: 0,
        fullness_mask: 0,
        changed_mask: 0,
        data: unsafe { std::mem::MaybeUninit::<[std::mem::MaybeUninit<Destroyed>; 64]>::uninit().assume_init() },
    };

    pub(super) static DEFAULT_PAGE: Page<Destroyed> = Page {
        presence_mask: 0,
        fullness_mask: 0,
        changed_mask: 0,
        count: 0,
        data: [&DEFAULT_CHUNK as *const Chunk<Destroyed> as *mut Chunk<Destroyed>; 64],
    };
}

impl Component for Destroyed {
    fn id() -> u32 {
        unsafe { __backstep_component_Destroyed::ID }
    }

    fn get_default_chunk() -> *const Chunk<Self> {
        &__backstep_component_Destroyed::DEFAULT_CHUNK
    }

    fn get_default_page() -> *const Page<Self> {
        &__backstep_component_Destroyed::DEFAULT_PAGE
    }

    fn initialize(id: u32) {
        unsafe {
            if __backstep_component_Destroyed::ID == u32::MAX {
                __backstep_component_Destroyed::ID = id;
            }
        }
    }

    fn schedule_cleanup_system(world: &mut World) {
        let sys =
            TemporaryComponentCleanupSystem::<Destroyed, crate::world::DestroyGroup>::new(world);
        world.scheduler_mut().add_system(sys);
    }
}

