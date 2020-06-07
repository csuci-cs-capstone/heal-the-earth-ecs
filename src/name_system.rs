use crate::system::system::*;
use crate::world::world::*;
use crate::name_component::*;
use crate::component::component_manager::*;
use crate::query::query::*;

pub struct NameSystem {
    query: Query
}

impl NameSystem {
    pub fn new() -> NameSystem {
        NameSystem {
            query: Query::new(QueryElement::read::<NameComponent>())
        }
    }
}

impl System for NameSystem {
    fn query(&self) -> &Query {
        &self.query
    }

    fn run(&mut self, w: &World, _dt: f32) {
        let names = w.manager::<NameComponent>().unwrap();
        let names = downcast_read_lock::<NameComponent>(&names);

        for n in names.iter() {
            println!("{}", n.name);
        }
    }
}
