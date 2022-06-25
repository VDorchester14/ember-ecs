use ember_ecs::core::component::Component;
use serde::Serialize;
use ron;
use ron::ser::{
    PrettyConfig,
    to_writer,
    Serializer as RonSerializer,
};
use std::fs::File;

#[derive(Serialize)]
pub struct Testf64Comp{
    value: f64
}

impl Component for Testf64Comp{}

#[test]
pub fn serialize_comp(){
    let c = Testf64Comp{value: 10.0};
    let pretty = PrettyConfig::new()
        .depth_limit(2)
        .separate_tuple_members(true)
        .enumerate_arrays(true);
    let writer = File::create("serialize_comp.ron").unwrap();
    let mut serializer = ron::ser::Serializer::new(&writer, Some(pretty.clone()), true).expect("Couldn't create ron serializer.");
    to_writer(&writer, &c);
}