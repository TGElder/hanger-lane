extern crate num;
#[cfg(test)] #[macro_use] extern crate hamcrest;

use num::Num;
use std::cmp::max;
use std::collections::HashSet;
use std::hash::Hash;

pub struct Edge<T: Num> {
    from: u32,
    to: u32,
    cost: T,
}

impl <T: Num> Edge<T> {

    pub fn new(from: u32, to: u32, cost: T) -> Edge<T> {
        Edge {
            from,
            to,
            cost }
    }
}

pub struct Network<T: Num> {
    pub nodes: u32,
    pub edges: Vec<Edge<T>>,
    below: Vec<Vec<u32>>,
    //above: Vec<Vec<T>>,
    //edges_in: Vec<Edge<T, U>>,
    //edges_out: Vec<Edge<T, U>>,
}

impl <T: Num> Network<T> {

    pub fn new(edges: Vec<Edge<T>>) -> Network<T> {

        let nodes = edges.iter().map(|e| max(e.from, e.to)).max().unwrap() + 1;
        let below = Network::calculate_belows(nodes, &edges) ;

        Network {
           nodes,
           edges,
           below,
        }
    }


    fn calculate_belows(nodes: u32, edges: &Vec<Edge<T>>) -> Vec<Vec<u32>> {
        (0..nodes).map(|n| Network::calculate_below(n, edges)).collect()
    }

    fn calculate_below(node: u32, edges: &Vec<Edge<T>>) -> Vec<u32> {
        edges.iter().filter(|e| e.from == node).map(|e| e.to).collect()
    }

    pub fn get_below(&self, node: u32) -> &Vec<u32> {
        &self.below[node as usize]
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

    fn get_test_network() -> Network<u8> {
        
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
        assert_that!(&network.get_below(0), contains(vec![1, 2]).exactly());
        assert_that!(&network.get_below(1), contains(vec![3]).exactly());
        assert_that!(&network.get_below(2), contains(vec![3]).exactly());
        assert_that!(network.get_below(3).len(), is(equal_to(0)));
        assert_that!(network.get_below(4).len(), is(equal_to(0)));
        assert_that!(&network.get_below(5), contains(vec![6]).exactly());
        assert_that!(&network.get_below(6), contains(vec![5]).exactly());
        assert_that!(&network.get_below(7), contains(vec![7]).exactly());
    }
}

