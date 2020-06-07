use std::any::*;
use crate::world::world::*;
use crate::system::system::*;

use std::vec::Vec;
use std::collections::{HashMap};

/*
struct SystemGraphNode {
    system_index: usize,
    depends_indices: Vec<usize>
}

pub struct NodeHandle<'a> {

}
*/

///Organizes the sequential execution of Systems and maintains meta-information about their states.
///
///The SystemManager is responsible for ordering the execution of Systems and maintaining information
///about whether or not they are disabled. When the engine is modified to run in parallel, SystemManager
///will also be responsible for working out which Systems can run in parallel and dispatching Systems
///to different threads. It will also keep track of which Systems need to run on the main thread,
///for example any System that works directly with input or touching OpenGL state.
pub struct SystemManager {
    systems: Vec<Box<dyn System>>,
    enabled: Vec<bool>,

    sys_map: HashMap<TypeId, usize>,

    /*
    indir_table: Vec<usize>,
    dep_graph: Vec<SystemGraphNode>
    */
}

impl SystemManager {

    ///Initializes a new, empty SystemManager.
    pub fn new() -> SystemManager {
        let sys_vec: Vec<Box<dyn System>> = Vec::new();
        let map: HashMap<TypeId, usize> = HashMap::new();
        let enabled: Vec<bool> = Vec::new();

        SystemManager {
            systems: sys_vec,
            enabled: enabled,
            sys_map: map
        }
    }

    ///Adds a System at the end of storage, registers it in the system map, and enables it.
    pub fn append<S: System>(&mut self, sys: S) {
        self.sys_map.insert(TypeId::of::<S>(), self.systems.len());
        self.systems.push(Box::new(sys));
        self.enabled.push(true);
    }

    ///Marks a given System as disabled.
    pub fn disable<S: System>(&mut self) {
        if self.sys_map.contains_key(&TypeId::of::<S>()) {
            self.enabled[self.sys_map[&TypeId::of::<S>()]] = false;
        }
    }

    ///Marks a system as enabled.
    pub fn enable<S: System>(&mut self) {
        if self.sys_map.contains_key(&TypeId::of::<S>()) {
            self.enabled[self.sys_map[&TypeId::of::<S>()]] = true;
        }
    }

    ///Dispatches all Systems in the order they were inserted, skipping disabled Systems.
    pub fn execute(&mut self, world: &World, dt: f32) {
        for i in 0..self.systems.len() {
            if self.enabled[i] {
                self.systems[i].run(world, dt);
            }
        }
    }
}
