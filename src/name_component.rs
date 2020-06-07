pub use crate::component::component::*;
pub use crate::component::component_manager::*;
use crate::common::generational_id::*;

use std::vec::*;
use std::collections::HashMap;
use std::string::*;
use std::slice::{Iter, IterMut};
use std::option::*;

#[derive(Clone, Debug)]
pub struct NameComponent {
    owner: GenerationalId,
    pub name: String
}

impl NameComponent {
    pub fn new(name: String) -> NameComponent {
        NameComponent {
            owner: GenerationalId::new(0, 0),
            name: name
        }
    }
}

impl Component for NameComponent {
    fn get_owner(&self) -> GenerationalId {
        self.owner
    }

    fn set_owner(&mut self, owner: GenerationalId) {
        self.owner = owner;
    }

    fn type_name(&self) -> String {
        let name: String = "NameComponent".to_string();
        name
    }

    fn text_repr(&self) -> String {
        let json_str = format!("{{\n\towner: {{\n\t\tid: {},\n\t\tgen: {}\n\t}},\n\tname: {}\n}}", self.owner.id, self.owner.gen, self.name);
        json_str
    }

    fn dynamic_clone(&self) -> Box<dyn Component> {
        Box::new(
            NameComponent {
                owner: self.owner,
                name: self.name.clone()
            }
        )
    }
}

#[derive(Debug)]
pub struct NameComponentManager {
    indir_map: HashMap<GenerationalId, usize>,
    components: Vec<NameComponent>,
    to_delete: Vec<usize>
}

impl NameComponentManager {
    pub fn new() -> NameComponentManager {
        NameComponentManager {
            indir_map: HashMap::new(),
            components: Vec::new(),
            to_delete: Vec::new()
        }
    }
}

impl ComponentManager for NameComponentManager {
    type Data = NameComponent;

    fn iter(&self) -> Iter<NameComponent> {
        self.components.iter()
    }

    fn iter_mut(&mut self) -> IterMut<NameComponent> {
        self.components.iter_mut()
    }

    fn fetch(&self, owner: GenerationalId) -> Option<&NameComponent> {
        if self.indir_map.contains_key(&owner) {
            return Some(&self.components[self.indir_map[&owner]]);
        } else {
            None
        }
    }

    fn fetch_mut(&mut self, owner: GenerationalId) -> Option<&mut NameComponent> {
        if self.indir_map.contains_key(&owner) {
            return Some(&mut self.components[self.indir_map[&owner]]);
        } else {
            None
        }
    }

    fn has_component(&self, owner: GenerationalId) -> bool {
        self.indir_map.contains_key(&owner)
    }

    fn insert(&mut self, owner: GenerationalId, value: NameComponent) -> Result<(), String> {
        if self.indir_map.contains_key(&owner) {
            return Err(format!("Cannot attach multiple of the same component to Entity {}", owner.id));
        }

        let ind = self.components.len();
        self.components.push(value);

        self.indir_map.insert(owner, ind);

        Ok(())
    }

    fn delete(&mut self, owner: GenerationalId) -> Result<(), String> {
        if self.indir_map.contains_key(&owner) {
            self.to_delete.push(self.indir_map[&owner] as usize);
            Ok(())
        } else {
            Err(format!("Entity {} does not have a NameComponent", owner.id))
        }
    }

    fn delete_now(&mut self, owner: GenerationalId) -> Result<(), String> {
        if self.indir_map.contains_key(&owner) {
            self.to_delete.sort_unstable();

            let d = self.indir_map[&owner] as usize;

            if self.to_delete.len() > 0 {
                let mut i = 0;
                let mut j = self.to_delete.len() - 1;
                let mut m = (j - i) / 2;

                while i <= j {
                    if self.to_delete[m] > d {
                        j = m;
                    } else if self.to_delete[m] < d {
                        i = m;
                    } else {
                        break;
                    }
                    m = (j - i) / 2;
                }

                if self.to_delete[m] == d {
                    self.to_delete.remove(m);
                    for i in m..self.to_delete.len() {
                        self.to_delete[i] -= 1;
                    }
                }
            } else {
                let _ = self.to_delete.pop();
            }

            self.components.remove(self.indir_map[&owner]);
            self.indir_map.remove(&owner);

            Ok(())

        } else {
            Err(format!("Entity {} does not have a NameComponent", owner.id))
        }
    }

    fn update(&mut self) {

        self.to_delete.sort_unstable();

        for i in 0..self.to_delete.len() {
            self.to_delete[i] -= i;
            self.indir_map.remove(&self.components[self.to_delete[i]].owner);
            self.components.remove(self.to_delete[i]);
        }

        self.to_delete.clear();
    }
}
