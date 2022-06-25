use ember_ecs::core::world::{World};
use ember_ecs::core::component::Component;

struct Testf64Comp{
    x: f64,
    y: f64,
    z: f64,
}

struct TestHealthComp{
    current_health: f64,
}

pub fn assert_component<T: Component>(){}

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

    world.add_component_to_entity::<Testf64Comp>(e1, Testf64Comp{x: 0.0, y: 1.0, z: 0.0});
    assert!(true);  // assuming it didn't crash. will learn better asserts later.
}

#[test]
fn query_test(){
    let mut world = World::new();
    let e1 = world.spawn();

    world.add_component_to_entity::<Testf64Comp>(e1, Testf64Comp{x: 0.0, y: 1.0, z: 0.0});
    world.add_component_to_entity::<TestHealthComp>(e1, TestHealthComp{current_health: 50.0});

    // let q = (TestHealthComp, Testf64Comp);
    struct Query{

    };
    let mut healths = world.borrow_component_vec::<TestHealthComp>().unwrap();
    let mut f64s = world.borrow_component_vec::<Testf64Comp>().unwrap();
    let zip = healths.iter_mut().zip(f64s.iter_mut());
    let iter = zip.filter_map(
        |(health, float64)|
        Some((
            health.as_mut()?,
            float64.as_mut()?
        ))
    );

    for (health, float64) in iter
    {
        assert!(health.current_health == 50.0);
        assert!(float64.x == 0.0);
    }

}

#[test]
fn can_impl_component(){
    impl Component for TestHealthComp{};
    assert_component::<TestHealthComp>();
}
