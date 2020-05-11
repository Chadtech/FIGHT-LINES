use crate::view::element::{tag, Element};

////////////////////////////////////////////////////////////////
// Types //
////////////////////////////////////////////////////////////////

pub struct Cell<MSG> {
    children: Vec<Element<MSG>>,
}

////////////////////////////////////////////////////////////////
// API //
////////////////////////////////////////////////////////////////

pub fn cell<MSG>(children: Vec<Element<MSG>>) -> Cell<MSG> {
    Cell { children }
}

impl<T> Cell<T> {
    pub fn view(self) -> Element<T> {
        tag("cell", Vec::new(), self.children)
    }
}
