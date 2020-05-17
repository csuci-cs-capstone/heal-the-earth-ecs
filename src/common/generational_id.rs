use std::hash::{Hash, Hasher};
use std::cmp::{PartialEq, PartialOrd, Eq, Ord, Ordering};

///Generational ID to manage ambiguous references to a given ID using a generational index.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct GenerationalId {
    pub id : u32,
    pub gen: u32
}

impl GenerationalId {
    ///Creates a new GenerationalId with id and gen.
    pub fn new(id: u32, gen: u32) -> GenerationalId {
        GenerationalId {
            id: id,
            gen: gen
        }
    }

}

impl Hash for GenerationalId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let complete: u64 = ((self.id as u64) << 32) | ((self.gen as u64));
        complete.hash(state);
    }
}

impl PartialEq for GenerationalId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.gen == other.gen
    }
}

impl Eq for GenerationalId {}

impl PartialOrd for GenerationalId {
    fn partial_cmp(&self, other: &GenerationalId) -> Option<Ordering> {
        if self.id < other.id {
            Some(Ordering::Less)
        } else if self.id > other.id {
            Some(Ordering::Greater)
        } else {
            if self.gen < other.gen {
                Some(Ordering::Less)
            } else if self.gen > other.gen {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        }
    }
}

impl Ord for GenerationalId {
    fn cmp(&self, other: &GenerationalId) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
