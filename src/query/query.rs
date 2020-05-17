use std::any::*;
use crate::component::component::*;

///Defines the type of access for a given QueryElement.
#[derive(Copy, Clone, Debug)]
pub enum QueryAccess {
    Check,
    Read,
    ReadWrite
}

///A single part of a larger query, giving the acceess method and TypeId of the Component.
#[derive(Copy, Clone, Debug)]
pub struct QueryPart {
    pub acc: QueryAccess,
    pub comp: TypeId
}

impl QueryPart {
    ///Constructs a new QueryPart using the TypeId of T
    pub fn new<T: Component>(a: QueryAccess) -> QueryPart {
        QueryPart {
            acc: a,
            comp: TypeId::of::<T>()
        }
    }
}

///A full query element describing the conditions of a section of a query (can be a deep recrusion,
///but shouldn't be).
#[derive(Clone, Debug)]
pub enum QueryElement {
    Part(QueryPart),

    Not(Box<QueryElement>),
    Or(Box<QueryElement>, Box<QueryElement>),
    And(Box<QueryElement>, Box<QueryElement>),
    Xor(Box<QueryElement>, Box<QueryElement>)
}

#[allow(dead_code)]
impl QueryElement {

    ///Returns a Part with QueryAccess::Check.
    pub fn has<T: Component>() -> QueryElement {
        QueryElement::Part(QueryPart::new::<T>(QueryAccess::Check))
    }

    ///Returns a Part with QueryAccess::Read.
    pub fn read<T: Component>() -> QueryElement {
        QueryElement::Part(QueryPart::new::<T>(QueryAccess::Read))
    }

    ///Returns a Part with QueryAccess::ReadWrite.
    pub fn read_write<T: Component>() -> QueryElement {
        QueryElement::Part(QueryPart::new::<T>(QueryAccess::ReadWrite))
    }

    ///Returns Not(q), representing the negation of q as a set.
    pub fn not(q: QueryElement) -> QueryElement {
        QueryElement::Not(Box::new(q))
    }

    ///Returns Or(l, r), representing the union of l and r as sets.
    pub fn or(l: QueryElement, r: QueryElement) -> QueryElement {
        QueryElement::Or(Box::new(l), Box::new(r))
    }

    ///Returns And(l, r), representing the intersection or l and r as sets.
    pub fn and(l: QueryElement, r: QueryElement) -> QueryElement {
        QueryElement::And(Box::new(l), Box::new(r))
    }

    ///Returns Xor(l, r), representing symmetric difference or disjunctive union of l and r as sets.
    ///Equivalent to And(Or(l, r), Not(And(l, r))
    pub fn xor(l: QueryElement, r: QueryElement) -> QueryElement {
        QueryElement::Xor(Box::new(l), Box::new(r))
    }
}

///Represents a full Query to a World for all Entities whose Components fulfill the conditions of the Query.
///Implicitly an And of all QueryElements added to it.
#[derive(Clone, Debug)]
pub struct Query {
    pub query: Vec<QueryElement>
}

#[allow(dead_code)]
impl Query {

    ///Creates a new Query with the provided QueryElement.
    pub fn new(qe: QueryElement) -> Query {
        let mut v = Vec::new();
        v.push(qe);
        Query {
            query: v
        }
    }

    ///Adds the provided QueryElement to the full Query, implicitly And'ing it with the rest of the Query.
    pub fn and(&mut self, qe: QueryElement) -> &mut Query {
        self.query.push(qe);
        self
    }

}
