use std::rc::Rc;
use std::cell::RefCell;
use rand::seq::SliceRandom;
use crate::util::{START_X, START_Y};

// Node struct for Recursive Backtracker
// stores a visited flag and its position in the grid
struct Node {
    visited: bool,
    pos: (i32, i32)
}

// Grid size that applies to the algorithm
// (applies to both width and height)
pub const GRID_SIZE: usize = 8;

impl Node {
    pub fn new( x: i32, y: i32 ) -> Node {
        Node { visited: false, pos: (x, y) }
    }
}

// Recursive Backtracker structure
// holds a grid of cells (vector of vector) containing reference-counted pointers to each node
// max_iterations is the maximum amount of iterations before the algorithm returns

pub struct RecursiveBacktracker {
    cells: [Vec<Rc<RefCell<Node>>>; GRID_SIZE],
    max_iterations: Option<usize>,
}

impl RecursiveBacktracker {
    // Initialize cell grid and pointers to nodes
    pub fn new( max_iterations: Option<usize> ) -> RecursiveBacktracker {
        let mut cells : [Vec<Rc<RefCell<Node>>>; GRID_SIZE as usize] = Default::default();
        for y in 0 .. GRID_SIZE {
            for x in 0 .. GRID_SIZE {
                cells[ y ].push( Rc::new( RefCell::new( Node::new( x as i32, y as i32 ) ) ) );
            }
        }
        RecursiveBacktracker { cells, max_iterations, }
    }

    // Whether the neighbor is considered a valid cell in the grid
    pub fn valid_cell( &self, x: i32, y: i32 ) -> bool {
        x < self.cells.len() as i32 && y < self.cells.len() as i32 && x >= 0 && y >= 0
    }

    // Carves a path at x, y to a random, unvisited neighbor cell
    // Layout is built throughout the recursion process, adding the valid x,y
    // coordinates for each visited cell
    pub fn carve_path( &self, x: i32, y: i32, layout: &mut Vec<(i32, i32)> ) {
        // If we exceed the max iteration count, return
        match self.max_iterations {
            Some( max ) => {
                if layout.len() >= max {
                    return;
                }
            },
            None => {},
        }

        // Borrow the cell, mark it as visited, and add its coordinate to the layout vector
        let cell = &self.cells[ y as usize ][ x as usize ];
        cell.borrow_mut().visited = true;
        layout.push( (x, y) );

        // Shuffle the vector of possible directions (up, down, left, right)
        let mut rng = rand::thread_rng();
        let mut directions = vec![ (0, -1), (1, 0), (0, 1), (-1, 0) ];
        directions.shuffle( &mut rng );

        // For each direction...
        for direction in directions.iter() {
            let new_loc = (x + direction.0, y + direction.1);
            // If the cell is valid, check if it is visited
            // If not, carve in that direction
            if self.valid_cell( new_loc.0, new_loc.1 ) {
                let next_cell = &self.cells[ new_loc.1 as usize ][ new_loc.0 as usize ];
                if !next_cell.borrow_mut().visited {
                    self.carve_path( new_loc.0, new_loc.1, layout );
                }
            }
        }
    }

    // Runs the algorithm and returns a vector of 2D coordinate positions
    // marking the valid rooms in the floor grid.
    pub fn run( &self ) -> Vec<(i32, i32)> {
        let mut layout = Vec::new();
        self.carve_path( START_X, START_Y, &mut layout );
        layout
    }

    // Resets the structure for future use
    pub fn reset( &mut self ) {
        self.cells = Default::default();
        for y in 0 .. GRID_SIZE {
            for x in 0 .. GRID_SIZE {
                self.cells[ y ].push( Rc::new( RefCell::new( Node::new( x as i32, y as i32 ) ) ) );
            }
        }
    }
}