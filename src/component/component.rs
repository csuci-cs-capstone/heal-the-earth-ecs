use crate::common::generational_id::*;
use std::string::*;
use downcast_rs::*;

///The trait defining all Component types used in the ECS architecture.
///
///Components, at a bare minimum, may be tags i.e. they hold no data and are only used to mark
///an entity as having a specific behavior.
///Most Components will have some form of data, or even large sets of data, but crucially, they
///have no behavior on their own. They have an owner entity and data, but they do not update
///themselves or have any methods of their own besides the below access methods.
///
pub trait Component: DowncastSync {
    ///
    ///Retrieves the GenerationalId of the owner of this Component. This is used for e.g.
    ///determining that the owner Entity is still active.
    ///
    fn get_owner(&self) -> GenerationalId;

    ///
    ///Sets the owner GenerationalId for this Component. Used for attaching a pre-constructed
    ///Component to an Entity.
    ///
    fn set_owner(&mut self, id: GenerationalId);

    ///
    ///Returns a String that should match the concrete type of the Component. Useful for
    ///debugging.
    ///
    fn type_name(&self) -> String;

    ///
    ///Returns a text JSON representation of the concrete type. Used for serialization, debug, and
    ///other functions.
    ///
    fn text_repr(&self) -> String;
}
impl_downcast!(sync Component);
