use crate::entity;
use crate::entity::Entity;
use std::collections::HashMap;

pub struct EntityManager {
    entities: Vec<entity::Entity>,
    to_add: Vec<entity::Entity>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        EntityManager {
            entities: Vec::new(),
            to_add: Vec::new(),
        }
    }
    pub fn init() {}
    pub fn update() {}
    pub fn add_entities(tag: String) {}
    pub fn get_entities(&mut self) -> &Vec<Entity> {
        &self.entities
    }
    pub fn get_entities_by_component<T: 'static>(&mut self) -> Vec<&mut Entity> {
        let mut entt: Vec<&mut entity::Entity> = Vec::new();
        for mut entity in self.entities.iter_mut() {
            match entity.get_component::<T>() {
                Some(n)=> {entt.push(entity)},
                None => (),
            };
        }
        entt
    }
    pub fn get_entity() {}
    pub fn add_entity(&mut self) -> &mut entity::Entity {
        let en = entity::Entity::new();
        self.entities.push(en);
        let index = self.entities.len() - 1;
        &mut self.entities[index]
    }
    pub fn delete() {}
}
