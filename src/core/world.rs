use std::cell::{RefCell, RefMut};

use super::storages::ComponentVec;


pub struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    pub fn spawn(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        // iterate all component vecs and see if we get one of the desired type
        for component_vec in self.component_vecs.iter_mut() {
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
        self.component_vecs.push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn borrow_component_vec<ComponentType: 'static>(
        &self
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }
}