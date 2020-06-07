use crate::entity::entity::*;
use crate::component::component::*;
use crate::component::component_manager::*;
use crate::common::generational_id::*;

use std::string::*;
use std::str::*;
use std::vec::Vec;
use std::collections::{HashMap, VecDeque};
use std::any::*;
use std::sync::RwLock;
use std::u32;

///Manages Entitys and Components and is responsible for organizing the game world.
///
///Represents every thing, i.e. every game object and their associated components, within the
///game world. In other words, the "Entity-Component" portion of an "Entity-Component-System"
///architecture.
///
pub struct World {
    entities: Vec<Entity>,
    free_queue: VecDeque<GenerationalId>,
    component_managers: HashMap<TypeId, RwLock<Box<dyn GeneralComponentManager>>>,
    quit: bool
}

#[allow(dead_code)]
impl World {

    ///Creates a new World with a valid starting state.
    pub fn new() -> World {
        let ents: Vec<Entity> = Vec::new();
        let mut free: VecDeque<GenerationalId> = VecDeque::new();
        free.push_front(GenerationalId::new(0, 1));

        let comp_mans: HashMap<TypeId, RwLock<Box<dyn GeneralComponentManager>>> = HashMap::new();

        World {
            entities: ents,
            free_queue: free,
            component_managers: comp_mans,
            quit: false
        }
    }

    ///Generates a new Entity and returns its GenerationalId.
    pub fn spawn(&mut self) -> GenerationalId {
        let id = self.free_queue.pop_front().unwrap();

        if self.entities.len() > id.id as usize {
            self.entities[id.id as usize].id = id;
        } else {
            self.entities.push(Entity::new(id));
        }

        if self.free_queue.len() == 0 {
            self.free_queue.push_back(GenerationalId::new(self.entities.len() as u32, 1));
        }

        id
    }

    ///Removes an Entity from the World, if provided a valid GenerationalId.
    pub fn delete(&mut self, id: GenerationalId) -> Result<(), String> {
        if (id.id as usize) < self.entities.len() && id.gen == self.entities[id.id as usize].id.gen {

            if id.gen == std::u32::MAX {
                self.free_queue.push_front(GenerationalId::new(id.id, 1));
            } else {
                self.free_queue.push_front(GenerationalId::new(id.id, id.gen + 1));
            }

            for m in self.component_managers.values_mut() {
                let mut m = m.write().unwrap();
                let _ = m.general_delete_now(id);
            }

            self.entities[id.id as usize].id.gen = 0;

            Ok(())
        } else {
            Err(String::from_str("Not a valid Entity ID").unwrap())
        }
    }

    ///Returns whether or not a GenerationalId is active.
    pub fn is_alive(&self, id: GenerationalId) -> bool {
        if (id.id as usize) < self.entities.len() {
            self.entities[id.id as usize].id.gen == id.gen
        } else {
            false
        }
    }

    ///Registers a ComponentManager to the World.
    pub fn register_manager<T: ComponentManager>(&mut self, man: T) {
        let dyn_man: Box<dyn ComponentManager<Data = T::Data>> = Box::new(man);
        self.component_managers.insert(TypeId::of::<T::Data>(), RwLock::new(Box::new(dyn_man)));
    }

    ///Returns an immutable reference to a ComponentManager for the given type, if one exists.
    pub fn manager<T: Component>(&self) -> Option<std::sync::RwLockReadGuard<Box<dyn GeneralComponentManager>>> {
        if self.component_managers.contains_key(&TypeId::of::<T>()) {
            let m = self.component_managers.get(&TypeId::of::<T>()).unwrap();

            Some(m.read().unwrap())

        } else {
            None
        }
    }

    ///Returns a mutable reference to a Componentmanager for a given type, if one exists.
    pub fn manager_mut<T: Component>(&self) -> Option<std::sync::RwLockWriteGuard<Box<dyn GeneralComponentManager>>> {
        if self.component_managers.contains_key(&TypeId::of::<T>()) {
            let m = self.component_managers.get(&TypeId::of::<T>()).unwrap();

            Some(m.write().unwrap())

        } else {
            None
        }
    }

    ///Attaches a provided Component to the Entity with the given Id, if it exists.
    pub fn attach_component<T: Component>(&self, handle: GenerationalId, comp: T) -> Result<(), String> {

        if !self.is_alive(handle) {
            return Err(format!("Entity ID {} is not active", handle.id))
        }

        let manager = self.manager_mut::<T>();

        let mut manager = match manager {
            Some(man) => man,
            None =>  return Err(format!("No successfully registered ComponentManager for {}", comp.type_name()))
        };

        let manager = (*manager).downcast_mut::<Box<dyn ComponentManager<Data=T>>>().unwrap();

        let mut comp = comp;
        comp.set_owner(handle);
        manager.insert(handle, comp)
    }

    ///Returns a given Entity and its associated Components in trait object form, if it is active.
    pub fn clone_components_of(&self, handle: GenerationalId) -> Result<(Entity, Vec<Box<dyn Component>>), String> {
        if !self.is_alive(handle) {
            Err(format!("Entity ID {} is not active", handle.id))
        } else {

            let mut v = Vec::new();

            for m in self.component_managers.values() {
                let r = m.read().unwrap();
                match r.fetch_dyn(handle) {
                    Some(c) => v.push(c.dynamic_clone()),
                    None => continue
                };
            }

            Ok((self.entities[handle.id as usize], v))
        }
    }

    ///Performs any potentially deferred operations such as Entity creation or deletion and updates all ComponentManagers.
    pub fn update(&mut self) {
        for m in self.component_managers.values_mut() {
            let _ = m.write().unwrap().general_update();
        }

    }

    ///Returns whether or not the game should quit running.
    pub fn should_quit(&self) -> bool {
        self.quit
    }

}
