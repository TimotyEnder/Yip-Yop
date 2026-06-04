use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
use crate::ecs::component_system::component::Component;

pub struct GameObject {
    name: String,
    id: usize,
    components: HashMap<TypeId, Box<dyn Component>>,
}
impl GameObject {
    pub fn new(name: &str) -> Self {
        GameObject {
            name: name.to_owned(),
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            components: HashMap::new(),
        }
    }
    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }
    pub fn get_id(&self) -> usize {
        self.id
    }
    pub fn add_component<T: Component>(&mut self, component: T) {
        self.components
            .insert(TypeId::of::<T>(), Box::new(component));
    }
    pub fn get_component_mut<T: Component>(&mut self) -> Option<&mut T> {
        self.components
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed_value| boxed_value.as_any_mut().downcast_mut())
    }
    pub fn get_component<T: Component>(&self) -> Option<&T> {
        self.components
            .get(&TypeId::of::<T>())
            .and_then(|boxed_value| boxed_value.as_any().downcast_ref())
    }
    pub fn has_component<T: Component>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }
}
