use crate::{ArcLock, link::Link};


#[derive(Debug)]
pub struct LinkBuilder {
    name: String,

}

impl LinkBuilder {
    pub fn new_box(name: String) -> LinkBuilder {
        Self { name }
    }

    pub(crate) fn build(self) -> ArcLock<Link> {
        // Not sure How i wanna do this yet,
        // Maybe with colliders and visuals, stacking and calculating the always calculating the endpoint or not?       
    }
}