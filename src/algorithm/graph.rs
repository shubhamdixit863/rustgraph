// Graph implementation

use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph <T>{
    adjacent_list:HashMap<T,Vec<T>>

}


impl <T> Graph<T>
where T: std::cmp::Eq+std::hash::Hash+Clone{

    // Initiailize a new Graph

   pub fn new ()->Self{
         Graph{
            adjacent_list:HashMap::new()
        }
    }


    pub fn add_vertex(&mut self,vertex: T){

        // firstly entry(vertex) gets called ,if the entry is vacant (ie the vertext doesnt exists in the map)
        // or insert_ the value returned bty Vec::new()
        // if the entry is occupied
        self.adjacent_list.entry(vertex)
            .or_insert(Vec::new());

    }

    pub fn add_edge(&mut self,src:T ,dst:T ){
        if let Some(neighbors) =self.adjacent_list.get_mut(&src){
            neighbors.push(dst.clone())
        }else{
            self.adjacent_list.insert(src.clone(),vec![dst.clone()]);
        }
        self.add_vertex(dst.clone());


        if let Some(neighbors) =self.adjacent_list.get_mut(&dst){
            neighbors.push(src.clone())
        }else{
            self.adjacent_list.insert(dst.clone(),vec![src.clone()]);
        }
        self.add_vertex(src.clone());
    }
}