pub use std::cell::RefCell;
use crate::core::component::Component;
use serde::Serialize;
use std::mem::MaybeUninit;

pub trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None)
    }
}

pub struct VecStorage<T>(Vec<MaybeUninit<T>>);

pub struct ComponentVecStorage{
    pub component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl ComponentVecStorage {
    pub fn new() -> Self {
        ComponentVecStorage{
            component_vecs: Vec::new(),
        }
    }
}