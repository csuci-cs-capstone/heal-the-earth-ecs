use crate::world::world::*;
use crate::system::system_manager::*;

///Stores a World and SystemManager.
///
///Stores a World and SystemManager and coordinates access to each from the rest of the main program.
///The initialization function should be responsible for setting up any libraries that need to be
///initialized, such as SDL2 or OpenGl, as well as setting the initial state of the World and
///SystemManager.
#[allow(dead_code)]
pub struct Universe {
    world: World,
    systems: SystemManager
}

#[allow(dead_code)]
impl Universe {

    ///Creates a new Universe, with init_func initializing any library state and World and SystemmManager
    ///state.
    pub fn new<I>(init_func: I) -> Universe
        where I: FnOnce() -> (World, SystemManager)
    {
        let (w, sm) = init_func();

        Universe {
            world: w,
            systems: sm
        }
    }

    ///Gets a mutable reference to the held World.
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    ///Gets a mutable reference to the held SystemManager.
    pub fn systems_mut(&mut self) -> &mut SystemManager {
        &mut self.systems
    }

    ///Runs the SystemManager for one iteration.
    pub fn run_once(&mut self, dt: f32) {
        self.systems.execute(&mut self.world, dt);
    }
}
