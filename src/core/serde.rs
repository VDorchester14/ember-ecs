use crate::core::component::Component;
use crate::core::world::World;
use std::cell::RefCell;


pub struct SerdeEntity{
    pub id: usize,
    pub components: Vec<Box<dyn Component>>,
}

pub struct SerdeScene {
    pub entities: Vec<SerdeEntity>,
}

impl SerdeScene{
    fn from_world(world: &mut World) -> Self{

        // let live_entities = world.live_entities();
        // let mut serde_entities = Vec::new();
        // let mut comp_storage = world.component_storage();
        // serde_entities.push(1);
        // for entity_id in live_entities {
        //     let mut components = Vec::new();
        //     for comp_vec in comp_storage.component_vecs.iter() {
        //         if let Some(comp_vec) = comp_vec
        //         .as_any_mut()
        //         .downcast_mut::<RefCell<Vec<Option<dyn Component>>>>()
        //         {
        //             match (*comp_vec)[entity_id] {
        //                 Some(c) => components.push(Box::new(c.clone())),
        //                 None => (),
        //             }
        //             // comp_vec.get_mut()[entity] = Some(component);  // if so, add the component to at the given entity index in that component vec
        //             // return;
        //         }
        //         // match (*comp_vec)[entity_id] {
        //         //     Some(c) => components.push(Box::new(c.clone())),
        //         //     None => (),
        //         // }
        //     }
            
        // }

        SerdeScene {
            entities: Vec::new(),
        }
    }
}

pub struct SerdeSceneSerializer{
    pub scene: SerdeScene,
}

pub struct ComponentSerializer<C: Component>{
    component: C,
}