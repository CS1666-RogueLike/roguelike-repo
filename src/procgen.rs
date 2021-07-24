use std::rc::Rc;
use std::cell::RefCell;
use rand::seq::SliceRandom;

struct Node {
    visited: bool,
    pos: (i32, i32)
}

pub const GRID_SIZE: usize = 5;

impl Node {
    pub fn new( x: i32, y: i32 ) -> Node {
        Node { visited: false, pos: (x, y) }
    }
}

struct RecursiveBacktracker {
    cells: [Vec<Rc<RefCell<Node>>>; GRID_SIZE],
}

impl RecursiveBacktracker {
    pub fn new() -> RecursiveBacktracker {
        let mut cells : [Vec<Rc<RefCell<Node>>>; GRID_SIZE as usize] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        for y in 0 .. GRID_SIZE {
            for x in 0 .. GRID_SIZE {
                cells[ y ].push( Rc::new( RefCell::new( Node::new( x as i32, y as i32 ) ) ) );
            }
        }
        RecursiveBacktracker { cells, }
    }

    pub fn valid_cell( &self, x: i32, y: i32 ) -> bool {
        x < self.cells.len() as i32 && y < self.cells.len() as i32 && x >= 0 && y >= 0
    }

    pub fn carve_path( &self, x: i32, y: i32 ) {
        println!( "Cell at {}, {} is now visited", x, y );
        let cell = &self.cells[ y as usize ][ x as usize ];
        cell.borrow_mut().visited = true;

        let mut rng = rand::thread_rng();
        let mut directions = vec![ (0, -1), (1, 0), (0, 1), (-1, 0) ];
        directions.shuffle( &mut rng );
        for direction in directions.iter() {
            let new_loc = (x + direction.0, y + direction.1);
            if self.valid_cell( new_loc.0, new_loc.1 ) {
                let next_cell = &self.cells[ new_loc.1 as usize ][ new_loc.0 as usize ];
                if !next_cell.borrow_mut().visited {
                    self.carve_path( new_loc.0, new_loc.1 );
                }
            }
        }

        println!( "Backtracking..." );
    }

    pub fn run( &self ) {
        println!( "Starting Recursive Backtracking" );
        self.carve_path( 0, 0 );
        println!( "Done" )
    }
}

pub fn recursive_backtracker_test() {
    println!("Recursive Backtracker test!\n");
    let rb = RecursiveBacktracker::new();
    rb.run();
}