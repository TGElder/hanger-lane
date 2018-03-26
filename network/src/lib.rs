extern crate num;
#[cfg(test)] #[macro_use] extern crate hamcrest;

#[derive(PartialEq, Debug, Clone)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub cost: u8,
}

impl Edge {

    pub fn new(from: usize, to: usize, cost: u8) -> Edge {
        Edge {
            from,
            to,
            cost }
    }

    pub fn create_4_neighbour_deltas() -> Vec<(usize, usize)> {
        vec![(1, 0), (0, 1)]
    }
    
    pub fn create_8_neighbour_deltas() -> Vec<(usize, usize)> {
        vec![(1, 0), (1, 1), (0, 1)]
    }

    pub fn create_grid(width: usize, height: usize, cost: u8, neighbour_deltas: Vec<(usize, usize)>) -> Vec<Edge> {

        fn get_index(x: usize, y: usize, width: usize) -> usize {
            (y * width) + x
        }

        fn create_edge(x: usize, y: usize, width: usize, height: usize, delta: &(usize, usize), cost: u8) -> Vec<Edge> {
            let x_b = x + delta.0;
            let y_b = y + delta.1;
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

pub struct Network {
    pub nodes: usize,
    edges_out: Vec<Vec<Edge>>,
    edges_in: Vec<Vec<Edge>>,
}

impl Network {

    pub fn new(nodes: usize, edges: &Vec<Edge>) -> Network {

        let mut out = Network {
           nodes,
           edges_out: Vec::with_capacity(nodes),
           edges_in: Vec::with_capacity(nodes),
        };

        out.calculate_all_edges_in_and_out(edges);

        out
    }

    fn calculate_all_edges_in_and_out(&mut self, edges: &Vec<Edge>) {
        self.edges_out = Vec::with_capacity(self.nodes);
        self.edges_in = Vec::with_capacity(self.nodes);
        for _ in 0..self.nodes {
            self.edges_out.push(vec![]);
            self.edges_in.push(vec![]);
        }
        for edge in edges {
            self.edges_out.get_mut(edge.from).unwrap().push(edge.clone());
            self.edges_in.get_mut(edge.to).unwrap().push(edge.clone());
        }
    }

    pub fn get_in(&self, node: usize) -> &Vec<Edge> {
        &self.edges_in[node]
    }

    pub fn get_out(&self, node: usize) -> &Vec<Edge> {
        &self.edges_out[node]
    }

    pub fn dijkstra(&self, nodes: Vec<usize>) -> Vec<Option<u32>> {
        use std::collections::BinaryHeap;
        use std::cmp::Ordering;

        #[derive(Eq)]
        struct Node {
            index: usize,
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

        let mut closed: Vec<bool> = vec![false; self.nodes];
        let mut out: Vec<Option<u32>> = vec![None; self.nodes];
        let mut heap = BinaryHeap::new();

        for node in nodes {
            heap.push(Node{ index: node, cost: 0 });
        }

        while let Some(Node {index, cost}) = heap.pop() {
            if !closed[index] {
                closed[index] = true;
                out[index] = Some(cost);

                for edge in self.get_in(index) {
                    if !closed[edge.from] {
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

    fn get_test_network(edges: &Vec<Edge>) -> Network {
        Network::new(8, edges)
    }

    #[test]
    fn test_get_out() {
        let edges = get_test_edges();
        let network = get_test_network(&edges);
        assert_that!(&network.get_out(0).iter().collect(), contains(vec![&edges[0], &edges[1], &edges[2]]).exactly());
        assert_that!(&network.get_out(1).iter().collect(), contains(vec![&edges[3]]).exactly());
        assert_that!(&network.get_out(2).iter().collect(), contains(vec![&edges[4], &edges[5]]).exactly());
        assert_that!(network.get_out(3).len(), is(equal_to(0)));
        assert_that!(network.get_out(4).len(), is(equal_to(0)));
        assert_that!(&network.get_out(5).iter().collect(), contains(vec![&edges[6]]).exactly());
        assert_that!(&network.get_out(6).iter().collect(), contains(vec![&edges[7], &edges[8]]).exactly());
        assert_that!(&network.get_out(7).iter().collect(), contains(vec![&edges[9]]).exactly());
    }

    #[test]
    fn test_get_in() {
        let edges = get_test_edges();
        let network = get_test_network(&edges);
        assert_that!(network.get_in(0).len(), is(equal_to(0)));
        assert_that!(&network.get_in(1).iter().collect(), contains(vec![&edges[0]]).exactly());
        assert_that!(&network.get_in(2).iter().collect(), contains(vec![&edges[1], &edges[2]]).exactly());
        assert_that!(&network.get_in(3).iter().collect(), contains(vec![&edges[3], &edges[4], &edges[5]]).exactly());
        assert_that!(network.get_in(4).len(), is(equal_to(0)));
        assert_that!(&network.get_in(5).iter().collect(), contains(vec![&edges[7], &edges[8]]).exactly());
        assert_that!(&network.get_in(6).iter().collect(), contains(vec![&edges[6]]).exactly());
        assert_that!(&network.get_in(7).iter().collect(), contains(vec![&edges[9]]).exactly());
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
        let network = get_test_network(&edges);
        let expected = vec![
            vec![Some(0), None, None, None, None, None, None, None],
            vec![Some(1), Some(0), None, None, None, None, None, None],
            vec![Some(2), None, Some(0), None, None, None, None, None],
            vec![Some(5), Some(4), Some(5), Some(0), None, None, None, None],
            vec![None, None, None, None, Some(0), None, None, None],
            vec![None, None, None, None, None, Some(0), Some(8), None],
            vec![None, None, None, None, None, Some(7), Some(0), None],
            vec![None, None, None, None, None, None, None, Some(0)]
        ];
        
        for i in 0..8 {
            assert_that!(&network.dijkstra(vec![i]), is(equal_to(&expected[i])));
        }
    }

    #[test]
    fn test_dijkstra_on_grid() {
        let edges = Edge::create_grid(4, 4, 1, Edge::create_4_neighbour_deltas());
        let network = Network::new(16, &edges);
        let expected = vec![
           Some(0), Some(1), Some(2), Some(3),
           Some(1), Some(2), Some(3), Some(4),
           Some(2), Some(3), Some(4), Some(5),
           Some(3), Some(4), Some(5), Some(6),
        ];
        
        assert_that!(&network.dijkstra(vec![0]), is(equal_to(&expected)));
    }

    #[test]
    fn test_multi_destinations() {
        let edges = Edge::create_grid(4, 4, 1, Edge::create_4_neighbour_deltas());
        let network = Network::new(16, &edges);
        let expected = vec![
           Some(0), Some(0), Some(0), Some(0),
           Some(1), Some(1), Some(1), Some(1),
           Some(2), Some(2), Some(2), Some(2),
           Some(3), Some(3), Some(3), Some(3),
        ];
        
        assert_that!(&network.dijkstra(vec![0, 1, 2, 3]), is(equal_to(&expected)));
    }
 
}

