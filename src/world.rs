use std::any::TypeId;
use std::collections::{HashMap, HashSet};

type EntityId = u32;

pub struct World {
    entities: HashMap<EntityId, Entity>,
    
}

pub struct Entity {
    id: EntityId,
    components: HashSet<TypeId>
}