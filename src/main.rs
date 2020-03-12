#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");


fn main() {
    let mut graph = unsafe {
      NetworKit_Graph::new(0, false, false)
    };
    unsafe {
      graph.addNode();
    };
      //graph.addNode();
      //graph.addNode();
      //graph.addEdge(0, 1, 0.0);
      //graph.addEdge(0, 2, 0.0);
      //println!("{:?}", graph.randomNeighbor(0));

    println!("Hello, world!");
}
