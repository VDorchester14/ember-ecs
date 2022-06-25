use std::any::Any;


pub trait Component: Any + Send + Sync + 'static {
    // type Storage: ComponentStorage;
}