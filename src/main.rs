use graphproject::algorithm::graph::Graph;
use graphproject::fileread::read;
struct User{
    name:String,
    id:i32,

}

fn main() {


let mut graph = Graph::new();
    graph.add_vertex("A");
    graph.add_vertex("B");
    graph.add_vertex("C");

    graph.add_edge("A","B");
    graph.add_edge("B","A");
    graph.add_edge("C","A");
    graph.add_edge("A","C");

    // user database


    read::file_read()



   // println!("{:?}",graph);

}
