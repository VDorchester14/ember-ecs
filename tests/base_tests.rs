use ember_ecs::core::world::{World};

#[test]
fn can_create_world() {
    let world = World::new();
    assert!(true);
}

#[test]
fn create_one_entity(){
    let mut world = World::new();
    let entity_1 = world.spawn();
    assert!(entity_1 == 0);
}

#[test]
fn create_10_entities(){
    let mut world = World::new();
    let mut entities = Vec::new();
    for i in 0..10 {
        entities.push(world.spawn());
    }
    for i in 0..10 {
        assert!(entities[i] == i);
    }
}

#[test]
fn create_f64_component_to_entity(){
    let mut world = World::new();
    let e1 = world.spawn();

    struct Testf64Comp{
        x: f64,
        y: f64,
        z: f64,
    };
    world.add_component_to_entity::<Testf64Comp>(e1, Testf64Comp{x: 0.0, y: 1.0, z: 0.0});
    assert!(true);  // assuming it didn't crash. will learn better asserts later.
}