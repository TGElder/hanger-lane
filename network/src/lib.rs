extern crate num;
#[cfg(test)] #[macro_use] extern crate hamcrest;

use num::Num;
use std::cmp::max;
use std::collections::HashSet;
use std::hash::Hash;

#[derive(PartialEq, Debug, Clone)]
pub struct Edge {
    from: u32,
    to: u32,
    cost: u8,
}

impl Edge {

    pub fn new(from: u32, to: u32, cost: u8) -> Edge {
        Edge {
            from,
            to,
            cost }
    }
}

pub struct Network {
    pub nodes: u32,
    pub edges: Vec<Edge>,
    edges_in: Vec<Vec<Edge>>,
    //edges_out: Vec<Vec<Edge<T>>>,
}

impl Network {

    pub fn new(edges: Vec<Edge>) -> Network {

        let nodes = edges.iter().map(|e| max(e.from, e.to)).max().unwrap() + 1;
        let edges_in = Network::calculate_all_edges_in(nodes, &edges) ;

        Network {
           nodes,
           edges,
           edges_in,
        }
    }

    fn calculate_all_edges_in(nodes: u32, edges: &Vec<Edge>) -> Vec<Vec<Edge>> {
        (0..nodes).map(|n| Network::calculate_edges_in(n, edges)).collect()
    }

    fn calculate_edges_in(node: u32, edges: &Vec<Edge>) -> Vec<Edge> {
        edges.iter().filter(|e| e.to == node).cloned().collect()
    }

    pub fn get_edges_in(&self, node: u32) -> &Vec<Edge> {
        &self.edges_in[node as usize]
    }

    //pub fn get_above(&self, node: T) -> Vec<T> {
    //    vec![]
    //}

    //pub fn get_in(&self, node: T) -> Vec<Edge<T, U>> {
    //    vec![]
    //}

    //pub fn get_out(&self, node: T) -> Vec<Edge<T, U>> {
    //    vec![]
    //}

    //pub fn get_edges(&self, node: T) -> Vec<Edge<T, U>> {
    //    vec![]
    //}

    //pub fn get_reverses(&self, node: T) -> Vec<Edge<T, U>> {
    //    vec![]
    //}
}


#[cfg(test)]
mod tests {

    use hamcrest::prelude::*;
    use {Edge, Network};

    fn get_test_network() -> Network {
        
        let edge_01 = Edge::new(0, 1, 1);
        let edge_02a = Edge::new(0, 2, 1);
        let edge_02b = Edge::new(0, 2, 1);
        let edge_13 = Edge::new(1, 3, 1);
        let edge_23a = Edge::new(2, 3, 1);
        let edge_23b = Edge::new(2, 3, 1);
        let edge_56 = Edge::new(5, 6, 1);
        let edge_65a = Edge::new(6, 5, 1);
        let edge_65b = Edge::new(6, 5, 1);
        let edge_77 = Edge::new(7, 7, 1);

        Network::new(vec![
            edge_01,
            edge_02a,
            edge_02b,
            edge_13,
            edge_23a,
            edge_23b,
            edge_56,
            edge_65a,
            edge_65b,
            edge_77,
        ])

    }

    #[test]
    fn test_nodes_count() {
        assert_eq!(get_test_network().nodes, 8);
    }

    #[test]
    fn test_get_below() {
        let network = get_test_network();
        assert_that!(network.get_edges_in(0).len(), is(equal_to(0)));
        assert_that!(&network.get_edges_in(1), contains(vec![Edge::new(0, 1, 1)]).exactly());
        assert_that!(&network.get_edges_in(2), contains(vec![Edge::new(0, 2, 1),
        Edge::new(0, 2, 1)]).exactly());
    }
}

