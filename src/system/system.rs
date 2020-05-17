use downcast_rs::*;
use crate::world::world::*;
use crate::query::query::*;

///
///The behavior part of an Entity-Component-System architecture. Responsible for implementing all
///Entity behaviors as well as side effects that depend on the Components of Entitys, e.g. rendering
///and sound.
///
///Systems provide a Query that is used by the SystemManager to figure out which Systems are capable
///of running in parallel without borring the same Components, or which systems need to run after
///other Systems e.g. if two Systems depend on particular Components, but one reads from them and
///the other writes to them.
///
pub trait System: DowncastSync {

    ///Returns a reference to the System's pre-defined Query, used for System management and querying
    ///the World for entities
    fn query(&self) -> &Query;

    ///Performs the operations relatied to the System and updates any caches or other internal data.
    fn run(&mut self, w: &mut World, dt: f32);
}
impl_downcast!(sync System);

impl<S: System> System for Box<S> {
    fn query(&self) -> &Query {
        <dyn System>::query(&**self)
    }

    fn run(&mut self, w: &mut World, dt: f32) {
        <dyn System>::run(&mut **self, w, dt);
    }
}
