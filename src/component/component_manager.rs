use std::slice::{Iter, IterMut};
use std::option::*;
use crate::component::component::*;
use crate::common::generational_id::*;

#[macro_use]
use downcast_rs::*;

///ComponentManager methods that do not rely on the specific Component type.
///
///A trait specifically for the methods all ComponentManagers must implement which don't rely on
///the specific type of Component they use. This is needed in order to work with all
///ComponentManagers in a single container, rather than separately storing each ComponentManager
///explicitly by concrete type.
///
pub trait GeneralComponentManager: DowncastSync {
    ///Fetches a given Component of an Entity as an immutable trait object.
    fn fetch_dyn(&self, owner: GenerationalId) -> Option<Box<& dyn Component>>;

    ///Fetches a given Component of an Entity as a mutable trait object.
    fn fetch_dyn_mut(&mut self, owner: GenerationalId) -> Option<Box<&mut dyn Component>>;

    ///Checks to see if the provided Entity is the owner of a Component in this ComponentManager.
    fn general_has_component(&self, owner: GenerationalId) -> bool;

    ///Deletes a Component from storage (allowed to be deferred).
    fn general_delete(&mut self, owner: GenerationalId) -> Result<(), String>;

    ///Deletes a Component from storage immediately.
    fn general_delete_now(&mut self, owner: GenerationalId) -> Result<(), String>;

    ///Updates the storage (if insertion or deletion has been deferred) and any non-Component
    ///internal variables, such as statistics or other metadata.
    fn general_update(&mut self);
}
impl_downcast!(sync GeneralComponentManager);

///Methods for adding Components to, deleting Components from, iterating over, querying, and updating
///ComponentManagers.
///
///Methods for accessing and managing Component storage. A ComponentManager acts as a basic
///storage, with no specified underlying type. It requires the ability to iterate over all
///Components stored by it, but also map-like access for fetching a specifc Component of a given
///Entity using its GeneraionalId. ComoponentManagers are allowed to store some internal data
///besides Components such as statistics, or metadata used for optimizing storage or access.
///
pub trait ComponentManager: DowncastSync + GeneralComponentManager + std::fmt::Debug {
    type Data: Component;

    ///Returns an iterator that iterates over every Component in storage immutably (i.e. read only).
    fn iter(&self) -> Iter<Self::Data>;

    ///Returns an iterator that iterates over every Component in storage mutably (i.e. read-write).
    fn iter_mut(&mut self) -> IterMut<Self::Data>;

    ///Immutably fetches a single Component attached to the Entity represented by owner, if it exists.
    fn fetch(&self, owner: GenerationalId) -> Option<&Self::Data>;

    ///Mutably fetches a single Component attached to the Entity represented by owner, if it exists.
    fn fetch_mut(&mut self, owner: GenerationalId) -> Option<&mut Self::Data>;

    ///Returns whether or not the given Entity has a Component in this Manager.
    fn has_component(&self, owner: GenerationalId) -> bool;

    ///Inserts a Component attached to owner into storage (allowed to be deferred if needed).
    fn insert(&mut self, owner: GenerationalId, value: Self::Data) -> Result<(), String>;

    ///Deletes a Component from storage (may be deferred, but should make that Component inaccessible).
    fn delete(&mut self, owner: GenerationalId) -> Result<(), String>;

    ///Deletes a Component from storage immediately. Not allowed to defer deletion.
    fn delete_now(&mut self, owner: GenerationalId) -> Result<(), String>;

    ///Executes any deferred operations and updates non-Component storage variables, if any.
    fn update(&mut self);
}
impl_downcast!(sync ComponentManager assoc Data where Data: Component);

impl<C: Component> ComponentManager for Box<dyn ComponentManager<Data=C>> {
    type Data = C;

    fn iter(&self) -> Iter<Self::Data> {
        <dyn ComponentManager<Data=C>>::iter(&**self)
    }

    fn iter_mut(&mut self) -> IterMut<Self::Data> {
        <dyn ComponentManager<Data=C>>::iter_mut(&mut **self)
    }

    fn fetch(&self, owner: GenerationalId) -> Option<&Self::Data> {
        <dyn ComponentManager<Data=C>>::fetch(&**self, owner)
    }

    fn fetch_mut(&mut self, owner: GenerationalId) -> Option<&mut Self::Data> {
        <dyn ComponentManager<Data=C>>::fetch_mut(&mut **self, owner)
    }

    fn has_component(&self, owner: GenerationalId) -> bool {
        <dyn ComponentManager<Data=C>>::has_component(&**self, owner)
    }

    fn insert(&mut self, owner: GenerationalId, value: Self::Data) -> Result<(), String> {
        <dyn ComponentManager<Data=C>>::insert(&mut **self, owner, value)
    }

    fn delete(&mut self, owner: GenerationalId) -> Result<(), String> {
        <dyn ComponentManager<Data=C>>::delete(&mut **self, owner)
    }

    fn delete_now(&mut self, owner: GenerationalId) -> Result<(), String> {
        <dyn ComponentManager<Data=C>>::delete_now(&mut **self, owner)
    }

    fn update(&mut self) {
        <dyn ComponentManager<Data=C>>::update(&mut **self);
    }
}

impl<CM: ComponentManager> GeneralComponentManager for CM {
    fn fetch_dyn(&self, owner: GenerationalId) -> Option<Box<& dyn Component>> {
        match self.fetch(owner) {
            Some(c) => Some(Box::new(c)),
            None => None
        }
    }

    fn fetch_dyn_mut(&mut self, owner: GenerationalId) -> Option<Box<&mut dyn Component>> {
        match self.fetch_mut(owner) {
            Some(c) => Some(Box::new(c)),
            None => None
        }
    }

    fn general_has_component(&self, owner: GenerationalId) -> bool {
        self.has_component(owner)
    }

    fn general_delete(&mut self, owner: GenerationalId) -> Result<(), String> {
        self.delete(owner)
    }

    fn general_delete_now(&mut self, owner: GenerationalId) -> Result<(), String> {
        self.delete_now(owner)
    }

    fn general_update(&mut self) {
        self.update()
    }
}
