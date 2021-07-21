use std::rc::Rc;
use std::cell::{RefCell, RefMut};

struct Edge {
    to: Rc<RefCell<Node>>,
}

impl Edge {
    pub fn new( to: Rc<RefCell<Node>> ) -> Edge {
        Edge { to, }
    }
}

struct Node {
    name: String,
    neighbors: Option<Vec<Edge>>,
    visited: bool,
}

impl Node {
    pub fn new( name: &str, neighbors: Option<Vec<Edge>> ) -> Node {
        Node { name: String::from( name ), neighbors: neighbors, visited: false, }
    }
}

struct Graph {
    nodes: Vec<Rc<RefCell<Node>>>,
}

impl Graph {
    pub fn new( nodes: Vec<Rc<RefCell<Node>>> ) -> Graph {
        Graph { nodes, }
    }
}

struct DepthFirstSearch {
    graph: Graph,
}

impl DepthFirstSearch {
    pub fn new( g: Graph ) -> DepthFirstSearch {
        DepthFirstSearch { graph: g }
    }

    pub fn visit_node( &self, n: &mut RefMut<'_, Node> ) {
        n.visited = true;
        println!( "{} is now visited", n.name );

        match &mut n.neighbors {
            Some( neighbors ) => {
                for neighbor in neighbors.iter_mut() {
                    let mut borrow_to = neighbor.to.borrow_mut();
                    if !borrow_to.visited {
                        println!( "Traversing edge" );
                        self.visit_node( &mut borrow_to );
                        println!( "Backtracking..." );
                    }
                }
            },
            None => {}
        }
    }

    pub fn run( &self ) {
        println!( "Starting DFS" );
        self.visit_node( &mut self.graph.nodes[ 0 ].borrow_mut() );
        println!( "Done" )
    }
}

pub fn dfs_test() {
    println!("DFS test!\n");

    // Nodes D, E, F, G
    let d = Rc::new( RefCell::new( Node::new( "D", None ) ) );
    let e = Rc::new( RefCell::new( Node::new( "E", None ) ) );
    let g = Rc::new( RefCell::new( Node::new( "G", None ) ) );

    let mut c_neighbors = Vec::new();
    c_neighbors.push( Edge::new( g.clone() ) );
    let c = Rc::new( RefCell::new( Node::new( "C", Some( c_neighbors ) ) ) );

    let f = Rc::new( RefCell::new( Node::new( "F", None ) ) );

    // Edge from B to C
    let mut b_neighbors = Vec::new();
    b_neighbors.push( Edge::new( c.clone() ) );
    let b = Rc::new( RefCell::new( Node::new( "B", Some( b_neighbors ) ) ) );

    // Edge from A to B, A to D
    let mut a_neighbors = Vec::new();
    a_neighbors.push( Edge::new( b.clone() ) );
    a_neighbors.push( Edge::new( d.clone() ) );
    let a = Rc::new( RefCell::new( Node::new( "A", Some( a_neighbors ) ) ) );

    let mut nodes = Vec::new();
    // First node in the list of nodes is the starting node (A)
    nodes.push( a );
    nodes.push( b );
    nodes.push( c );
    nodes.push( d );
    nodes.push( e );
    nodes.push( f );
    nodes.push( g );

    let graph = Graph::new( nodes );

    let dfs = DepthFirstSearch::new( graph );
    dfs.run();
}