use backstep_macros::Component;
use crate::entity::Entity;

#[derive(Component, Clone, Debug)]
pub struct Parent {
    pub num_child: usize,
    pub last_child: Entity
}
#[derive(Component, Clone, Debug)]
pub struct Child {
    pub parent: Entity,
    pub next: Entity,
    pub prev: Entity
}