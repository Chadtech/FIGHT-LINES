use seed::dom_entity_names::Tag;
use seed::prelude::*;
use std::borrow::Cow;

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Cell<MSG> {
    children: Vec<Node<MSG>>,
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

pub fn cell<MSG>(children: Vec<Node<MSG>>) -> Cell<MSG> {
    Cell { children }
}

pub fn single<MSG>(child: Node<MSG>) -> Cell<MSG> {
    cell(vec![child])
}

impl<T> Cell<T> {
    pub fn view(self) -> Node<T> {
        let mut element: El<T> = El::empty(Tag::Custom(Cow::Borrowed("cell")));
        element.children = self.children;
        Node::Element(element)
    }
}
