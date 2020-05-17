mod common;
mod component;
mod entity;
mod query;
mod world;
mod system;
mod universe;
mod name_component;
mod name_system;

use world::world::World;
use name_component::*;
use system::system_manager::*;
use name_system::*;

fn main() {
    let mut world = World::new();
    let mut sys_man = SystemManager::new();
    sys_man.append(NameSystem::new());
    world.register_manager(NameComponentManager::new());

    println!("Spawning 3 Entities...\n");

    let handle_1 = world.spawn();
    let handle_2 = world.spawn();
    let handle_3 = world.spawn();

    world.attach_component(handle_1, NameComponent::new("First".to_string())).unwrap();
    world.attach_component(handle_3, NameComponent::new("Third".to_string())).unwrap();

    println!("\n\nHandle 1: {:?}\nHandle 2: {:?}\nHandle 3: {:?}\n\n", handle_1, handle_2, handle_3);

    sys_man.execute(&mut world, 0.016f32);

    println!("\n\nDeleting 2 Entities...");

    world.delete(handle_1).unwrap();
    world.delete(handle_2).unwrap();

    println!("\n\nHandle 1 alive: {}\nHandle 3 alive: {}\nHandle 2 alive: {}\n\n", world.is_alive(handle_1), world.is_alive(handle_3), world.is_alive(handle_2));

    println!("Disabling NameSystem...\n");

    sys_man.disable::<NameSystem>();

    println!("Executing SystemManager...\n");

    sys_man.execute(&mut world, 0.016f32);

    println!("Enabling NameSystem...\n");

    sys_man.enable::<NameSystem>();

    println!("Executing SystemManager...\n");

    sys_man.execute(&mut world, 0.016f32);

    println!("\nSpawning 3 Entities...\n");

    let handle_2 = world.spawn();
    let handle_1 = world.spawn();
    let handle_4 = world.spawn();

    world.attach_component(handle_2, NameComponent::new("Second".to_string())).unwrap();

    println!("\n\nHandle 1: {:?}\nHandle 2: {:?}\nHandle 3: {:?}\nHandle 4: {:?}\n\n", handle_1, handle_2, handle_3, handle_4);

    sys_man.execute(&mut world, 0.016f32);

    println!("\n");

}
