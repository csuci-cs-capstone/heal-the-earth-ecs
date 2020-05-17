use crate::common::generational_id::*;
use std::option::*;
use std::cmp::{PartialEq, PartialOrd, Eq, Ord, Ordering};

///The type representing an in-game Entity.
///
///The Entity structure represents a single game object that may be a parent or child of any other
///entity within the game world. The ID of a given entity is, ideally, an index into an array or
///a key into a map, with a generation count to be sure that a reference to an older entity does
///not apply to a new entity allocated at the same index or using the same key
///
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Entity {
    pub id: GenerationalId,
}

impl Entity {

    ///Creates a new Entity with GenerationalId gen_id
    pub fn new(gen_id: GenerationalId) -> Entity {
        Entity {
            id: gen_id,
        }
    }
}

//An Entity is, itself, just its ID- if two IDs are equal, then they refer to the same Entity
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Entity {}

impl PartialOrd for Entity {
    fn partial_cmp(&self, other: &Entity) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Entity {
    fn cmp(&self, other: &Entity) -> Ordering {
        self.id.cmp(&other.id)
    }
}
