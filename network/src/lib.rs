extern crate num;
#[cfg(test)] #[macro_use] extern crate hamcrest;

use std::cmp::max;

#[derive(PartialEq, Debug)]
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

    pub fn create_4_neighbour_deltas() -> Vec<(u8, u8)> {
        vec![(1, 0), (0, 1)]
    }
    
    pub fn create_8_neighbour_deltas() -> Vec<(u8, u8)> {
        vec![(1, 0), (1, 1), (0, 1)]
    }

    pub fn create_grid(width: u32, height: u32, cost: u8, neighbour_deltas: Vec<(u8, u8)>) -> Vec<Edge> {

        fn get_index(x: u32, y: u32, width: u32) -> u32 {
            (y * width) + x
        }

        fn create_edge(x: u32, y: u32, width: u32, height: u32, delta: &(u8, u8), cost: u8) -> Vec<Edge> {
            let x_b = x + delta.0 as u32;
            let y_b = y + delta.1 as u32;
            if (x_b >= width) || (y_b >= height) {
                return vec![]
            }
            let index_a = get_index(x, y, width);
            let index_b = get_index(x_b, y_b, width);
            vec![Edge::new(index_a, index_b, cost), Edge::new(index_b, index_a, cost)]
        }

        neighbour_deltas.iter().flat_map(move |d| {
            (0..width).flat_map(move |x| {
                (0..height).flat_map(move |y| create_edge(x, y, width, height, d, cost))
            })
        }).collect()
    }


}

pub struct Network<'a> {
    pub nodes: u32,
    pub edges: &'a Vec<Edge>,
    edges_out: Vec<Vec<&'a Edge>>,
    edges_in: Vec<Vec<&'a Edge>>,
}

impl <'a> Network<'a> {

    pub fn new(nodes: u32, edges: &'a Vec<Edge>) -> Network<'a> {

        let edges_out = Network::calculate_all_edges_out(nodes, &edges) ;
        let edges_in = Network::calculate_all_edges_in(nodes, &edges) ;

        Network {
           nodes,
           edges,
           edges_out,
           edges_in,
        }
    }

    fn calculate_all_edges_in(nodes: u32, edges: &'a Vec<Edge>) -> Vec<Vec<&'a Edge>> {
        (0..nodes).map(|n| Network::calculate_edges_in(n, edges)).collect()
    }

    fn calculate_edges_in(node: u32, edges: &'a Vec<Edge>) -> Vec<&'a Edge> {
        edges.iter().filter(|e| e.to == node).collect()
    }

    pub fn get_in(&self, node: u32) -> &Vec<&'a Edge> {
        &self.edges_in[node as usize]
    }

    fn calculate_all_edges_out(nodes: u32, edges: &'a Vec<Edge>) -> Vec<Vec<&'a Edge>> {
        (0..nodes).map(|n| Network::calculate_edges_out(n, edges)).collect()
    }

    fn calculate_edges_out(node: u32, edges: &'a Vec<Edge>) -> Vec<&'a Edge> {
        edges.iter().filter(|e| e.from == node).collect()
    }

    pub fn get_out(&self, node: u32) -> &Vec<&'a Edge> {
        &self.edges_out[node as usize]
    }

    fn dijkstra(&self, node: u32) -> Vec<Option<u32>> {
        use std::collections::BinaryHeap;
        use std::cmp::Ordering;

        #[derive(Eq)]
        struct Node {
            index: u32,
            cost: u32,
        }

        impl Ord for Node {
            fn cmp(&self, other: &Node) -> Ordering {
                self.cost.cmp(&other.cost).reverse()
            }
        }

        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for Node {
            fn eq(&self, other: &Node) -> bool {
                self.cost == other.cost
            }
        }

        let mut closed: Vec<bool> = vec![false; self.nodes as usize];
        println!("{:?}", closed);
        let mut out: Vec<Option<u32>> = vec![None; self.nodes as usize];
        let mut heap = BinaryHeap::new();

        heap.push(Node{ index: node, cost: 0 });

        while let Some(Node {index, cost}) = heap.pop() {
            println!("On node {} with cost {}", index, cost);
            if !closed[index as usize] {
                closed[index as usize] = true;
                out[index as usize] = Some(cost);

                println!("Looking for edges");
                for edge in self.get_in(index) {
                    println!("Considering edge from {} to {} ({})", edge.from, edge.to, closed[edge.to as usize]);
                    if !closed[edge.from as usize] {
                        println!("Adding {} to heap with {}", edge.from, cost + edge.cost as u32);
                        heap.push(Node{ index: edge.from, cost: cost + edge.cost as u32 });
                    }
                }
            }
        }

        out
    }
}


#[cfg(test)]
mod tests {

    use hamcrest::prelude::*;
    use {Edge, Network};

    fn get_test_edges() -> Vec<Edge> {
        vec![Edge::new(0, 1, 1),
            Edge::new(0, 2, 2),
            Edge::new(0, 2, 3),
            Edge::new(1, 3, 4),
            Edge::new(2, 3, 5),
            Edge::new(2, 3, 6),
            Edge::new(5, 6, 7),
            Edge::new(6, 5, 8),
            Edge::new(6, 5, 9),
            Edge::new(7, 7, 10)]
    }

    fn get_test_network(nodes: u32, edges: &Vec<Edge>) -> Network {
        Network::new(nodes, edges)
    }

    #[test]
    fn test_get_out() {
        let edges = get_test_edges();
        let network = get_test_network(8, &edges);
        assert_that!(network.get_out(0), contains(vec![&edges[0], &edges[1], &edges[2]]).exactly());
        assert_that!(network.get_out(1), contains(vec![&edges[3]]).exactly());
        assert_that!(network.get_out(2), contains(vec![&edges[4], &edges[5]]).exactly());
        assert_that!(network.get_out(3).len(), is(equal_to(0)));
        assert_that!(network.get_out(4).len(), is(equal_to(0)));
        assert_that!(network.get_out(5), contains(vec![&edges[6]]).exactly());
        assert_that!(network.get_out(6), contains(vec![&edges[7], &edges[8]]).exactly());
        assert_that!(network.get_out(7), contains(vec![&edges[9]]).exactly());
    }

    #[test]
    fn test_get_in() {
        let edges = get_test_edges();
        let network = get_test_network(8, &edges);
        assert_that!(network.get_in(0).len(), is(equal_to(0)));
        assert_that!(network.get_in(1), contains(vec![&edges[0]]).exactly());
        assert_that!(network.get_in(2), contains(vec![&edges[1], &edges[2]]).exactly());
        assert_that!(network.get_in(3), contains(vec![&edges[3], &edges[4], &edges[5]]).exactly());
        assert_that!(network.get_in(4).len(), is(equal_to(0)));
        assert_that!(network.get_in(5), contains(vec![&edges[7], &edges[8]]).exactly());
        assert_that!(network.get_in(6), contains(vec![&edges[6]]).exactly());
        assert_that!(network.get_in(7), contains(vec![&edges[9]]).exactly());
    }

    #[test]
    fn test_create_grid() {
        
        let expected_edges = vec![Edge::new(0, 1, 1),
                                  Edge::new(0, 3, 1),
                                  Edge::new(1, 0, 1),
                                  Edge::new(1, 2, 1),
                                  Edge::new(1, 4, 1),
                                  Edge::new(2, 1, 1),
                                  Edge::new(2, 5, 1),
                                  Edge::new(3, 0, 1),
                                  Edge::new(3, 4, 1),
                                  Edge::new(3, 6, 1),
                                  Edge::new(4, 1, 1),
                                  Edge::new(4, 3, 1),
                                  Edge::new(4, 5, 1),
                                  Edge::new(4, 7, 1),
                                  Edge::new(5, 2, 1),
                                  Edge::new(5, 4, 1),
                                  Edge::new(5, 8, 1),
                                  Edge::new(6, 3, 1),
                                  Edge::new(6, 7, 1),
                                  Edge::new(7, 4, 1),
                                  Edge::new(7, 6, 1),
                                  Edge::new(7, 8, 1),
                                  Edge::new(8, 5, 1),
                                  Edge::new(8, 7, 1),
        ];

        let edges = Edge::create_grid(3, 3, 1, Edge::create_4_neighbour_deltas());
        assert_that!(&edges.iter().collect(), contains(expected_edges.iter().collect()).exactly());
    }

    #[test]
    fn test_dijkstra() {
        let edges = get_test_edges();
        let network = get_test_network(8, &edges);

        //assert_that!(&network.dijkstra(0), contains(vec![Some(0), None, None, None, None, None, None, None]).exactly().in_order());
        //assert_that!(&network.dijkstra(1), contains(vec![Some(1), Some(0), None, None, None, None, None, None]).exactly().in_order());
        //assert_that!(&network.dijkstra(2), contains(vec![Some(3), None, Some(0), None, None, None, None]).exactly().in_order());
        assert_that!(&network.dijkstra(3), contains(vec![Some(5), Some(4), Some(5), Some(0), None, None, None, None]).exactly());
        //assert_that!(&network.dijkstra(4), contains(vec![None, None, None, None, Some(0), None, None, None]).exactly().in_order());
        //assert_that!(&network.dijkstra(5), contains(vec![None, None, None, None, None, Some(0), Some(8), None]).exactly().in_order());
        //assert_that!(&network.dijkstra(6), contains(vec![None, None, None, None, None, Some(7), Some(0), None]).exactly().in_order());
        //assert_that!(&network.dijkstra(7), contains(vec![None, None, None, None, None, None, None, Some(0)]).exactly().in_order());
    }
}

