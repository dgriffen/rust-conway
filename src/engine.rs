use std::collections::HashMap;

use cell::State;
use world;

///The main implementation of Conway's game of life. This engine utilizes a list of updates to
///track interesting areas in the world. Only cells adjacent to cells that updated during the last
///generation are evaluated for a new state
pub struct GrifLife<T: world::World> {
    generation: uint,
    updated: HashMap<(int, int), State>,
    world: T
}

pub trait ConwayEngine<T: world::World> {
    
    ///Computes the next generation of cells, works on an internally held world::World object
    fn next_generation(&mut self);

    ///Return an immutable reference to the internally held world object. This is useful for
    ///getting access to the world so that It can be printed
    fn world_ref<'w>(&'w self) -> &'w T;

    ///Return a mutable reference to the internally held world object.
    fn world_ref_mut<'w>(&'w mut self) -> &'w mut T;

    ///Correct way to set the cell in the engine so we can know about the addition
    fn set_cell(&mut self, x: int, y: int);
}



impl<T: world::World>ConwayEngine<T> for GrifLife<T> {

    fn next_generation(&mut self) {
        //new list of updates
        let mut new_map = HashMap::new();
        let mut checked_map = HashMap::new();
        //loop through all the updated cells
        for (location, _) in self.updated.iter() {
            let (x, y) = *location;
            //check for new states on all adjacent cells
            for i in range(-1i, 2i) {
                for j in range(-1i, 2i) {
                    
                    //if this cell hasn't been checked already
                    if checked_map.get(&(x - i, y - j)) == None {
                        //get the current state
                        let adj_state = self.world.get_cell(x - i, y - j);
                        //get the new state for this cell
                        let new_adj_state = self.new_state((self.world.get_cell(x - i, y - j), (x - i, y - j)));

                        //if the cell changed, update the world and list
                        if adj_state != new_adj_state {
                            new_map.insert((x - i, y - j), new_adj_state);
                        }

                        //add this cell to the checked map
                        checked_map.insert((x - i, y - j), true);
                    }
                }
            }
        }

        //update the world with new list
        for (location, cell) in new_map.iter() {
            match (*location, *cell) {
                ((x, y), State::Dead)    => self.world.kill_cell(x, y),
                ((x, y), _)             => self.world.set_cell(x, y)
            }
        }
        self.updated = new_map;
        self.generation += 1;
    }

    fn world_ref<'w>(&'w self) -> &'w T {
        &self.world
    }

    fn world_ref_mut<'w>(&'w mut self) -> &'w mut T {
        &mut self.world
    }

    fn set_cell(&mut self, x: int, y: int) {
        self.updated.insert((x, y), State::Alive);
        self.world.set_cell(x, y);
    }
} 

impl<T: world::World> GrifLife<T> {

    ///Create a new instance of the engine, this should be used
    ///on a world with an initial setup of cells.
    pub fn new(world: T) -> GrifLife<T> {
        let mut first_map = HashMap::new();
        for(location, cell) in world.iter() {
            first_map.insert(*location, *cell);
        }
        GrifLife { generation: 1, updated: first_map, world: world}
    }

    //calculate the new cell state
    fn new_state(&self, cell: (State, (int, int))) -> State {
        let (state, (x, y)) = cell;

        //count of sourrounding live cells
        let mut count = 0i;

        //count the surrounding cells
        for i in range(-1i, 2i) {
            for j in range(-1i, 2i) {
                if i == 0 && j == 0 {
                    continue;
                }
                if self.world.get_cell(x - i, y - j) == State::Alive {
                    count += 1;
                }
            }
        }

        //apply conways rules
        if count == 3 {
            return State::Alive;
        }
        if count == 2 && state == State::Alive {
            return State::Alive;
        }
        State::Dead
    }
}
