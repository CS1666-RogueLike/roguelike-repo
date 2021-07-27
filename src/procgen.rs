use std::rc::Rc;
use std::cell::RefCell;
use rand::seq::SliceRandom;
use crate::util::{START_X, START_Y};

struct Node {
    visited: bool,
    pos: (i32, i32)
}

pub const GRID_SIZE: usize = 8;

impl Node {
    pub fn new( x: i32, y: i32 ) -> Node {
        Node { visited: false, pos: (x, y) }
    }
}

pub struct RecursiveBacktracker {
    cells: [Vec<Rc<RefCell<Node>>>; GRID_SIZE],
    max_iterations: Option<usize>,
}

impl RecursiveBacktracker {
    pub fn new( max_iterations: Option<usize> ) -> RecursiveBacktracker {
        let mut cells : [Vec<Rc<RefCell<Node>>>; GRID_SIZE as usize] = Default::default();
        for y in 0 .. GRID_SIZE {
            for x in 0 .. GRID_SIZE {
                cells[ y ].push( Rc::new( RefCell::new( Node::new( x as i32, y as i32 ) ) ) );
            }
        }
        RecursiveBacktracker { cells, max_iterations, }
    }

    pub fn valid_cell( &self, x: i32, y: i32 ) -> bool {
        x < self.cells.len() as i32 && y < self.cells.len() as i32 && x >= 0 && y >= 0
    }

    pub fn carve_path( &self, x: i32, y: i32, layout: &mut Vec<(i32, i32)> ) {
        match self.max_iterations {
            Some( max ) => {
                if layout.len() >= max {
                    return;
                }
            },
            None => {},
        }
        let cell = &self.cells[ y as usize ][ x as usize ];
        cell.borrow_mut().visited = true;
        layout.push( (x, y) );

        let mut rng = rand::thread_rng();
        let mut directions = vec![ (0, -1), (1, 0), (0, 1), (-1, 0) ];
        directions.shuffle( &mut rng );
        for direction in directions.iter() {
            let new_loc = (x + direction.0, y + direction.1);
            if self.valid_cell( new_loc.0, new_loc.1 ) {
                let next_cell = &self.cells[ new_loc.1 as usize ][ new_loc.0 as usize ];
                if !next_cell.borrow_mut().visited {
                    self.carve_path( new_loc.0, new_loc.1, layout );
                }
            }
        }
    }

    pub fn run( &self ) -> Vec<(i32, i32)> {
        let mut layout = Vec::new();
        self.carve_path( START_X, START_Y, &mut layout );
        layout
    }

    pub fn reset( &mut self ) {
        self.cells = Default::default();
        for y in 0 .. GRID_SIZE {
            for x in 0 .. GRID_SIZE {
                self.cells[ y ].push( Rc::new( RefCell::new( Node::new( x as i32, y as i32 ) ) ) );
            }
        }
    }
}