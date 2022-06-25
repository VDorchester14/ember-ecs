use std::cell::{RefCell, RefMut};
use std::borrow::BorrowMut;

use super::storages::{
    ComponentVec,
    ComponentVecStorage,
};

pub struct World {
    entities_count: usize,
    live_entities: Vec<usize>,
    // component_vecs: Vec<Box<dyn ComponentVec>>,
    component_storage: ComponentVecStorage,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            // component_vecs: Vec::new(),
            component_storage: ComponentVecStorage::new(),
            live_entities: Vec::new(),
        }
    }

    pub fn spawn(&mut self) -> usize {
        let entity_id = self.entities_count;
        // for component_vec in self.component_vecs.iter_mut() {
        //     component_vec.push_none();
        // }
        for component_vec in self.component_storage.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        self.live_entities.push(entity_id.clone());
        entity_id
    }

    pub fn despawn(&mut self) {
        todo!();
    }

    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        // iterate all component vecs and see if we get one of the desired type
        for component_vec in self.component_storage.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                component_vec.get_mut()[entity] = Some(component);  // if so, add the component to at the given entity index in that component vec
                return;
            }
        }
        
        // No matching component storage exists yet, so we have to make one.
        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        // All existing entities don't have this component, so we give them `None`
        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        // Give this Entity the Component.
        new_component_vec[entity] = Some(component);
        self.component_storage.component_vecs.push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn borrow_component_vec<ComponentType: 'static>(
        &self
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_storage.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }

    pub fn live_entities(&self) -> Vec<usize> {
        self.live_entities.clone()
    }

    pub fn component_storage(&mut self) -> &mut ComponentVecStorage {
        self.component_storage.borrow_mut()
    }
}