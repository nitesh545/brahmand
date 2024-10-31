use crate::components;
use rand::Rng;
use sfml::graphics::CircleShape;
use crate::component_manager::ComponentManager;

pub struct Entity {
    active: bool,
    id: u32,
    components: ComponentManager,
}

impl Entity {
    pub fn new() -> Entity {
        let random_u32 = rand::thread_rng().gen_range(1..99999);
        Entity {
            active: true,
            id: random_u32,
            components: ComponentManager::new(),
        }
    }

    pub fn destroy(mut self) {
        self.active = false;
    }
    pub fn get_active(self) -> bool {
        self.active
    }
    pub fn get_id(self) -> u32 {
        self.id
    }
    pub fn add_component<T: 'static>(&mut self, component: T) {
        self.components.add_component(component);
    }
    pub fn remove_component<T: 'static>(&mut self) -> Option<Box<T>> {
        self.components.remove_component::<T>()
    }
    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        self.components.get_component::<T>()
    }
    pub fn get_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.components.get_component_mut::<T>()
    }
}
