use std::any::{Any, TypeId};
use std::collections::HashMap;


pub struct ComponentManager {
    components: HashMap<TypeId, Box<dyn Any>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        ComponentManager{components: HashMap::new()}
    }

    pub fn add_component<T: 'static>(&mut self, component: T) {
        self.components.insert(TypeId::of::<T>(), Box::new(component));
    }

    pub fn remove_component<T: 'static>(&mut self) -> Option<Box<T>> {
        self.components.remove(&TypeId::of::<T>()).and_then(|c| {c.downcast::<T>().ok()})
    }

    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        self.components.get(&TypeId::of::<T>()).and_then(|c| c.downcast_ref())
    }
    
    pub fn get_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.components.get_mut(&TypeId::of::<T>()).and_then(|c| c.downcast_mut())
    }
}