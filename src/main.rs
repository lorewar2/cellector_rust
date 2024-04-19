mod poa;
mod lcskgraphefficient;
mod bit_tree;
use std::collections::HashMap;

use poa::*;
use petgraph::dot::Dot;
use petgraph::visit::Topo;
use crate::lcskgraphefficient::{simple_dfs_all_paths, find_kmer_matches, lcskpp_graph};

fn main() {
    // test run the lcsk++ incomplete code
    let x = b"CTATAGAGTA".to_vec();
    let y = b"ATTATG".to_vec();
    let aligner = Aligner::new(2, -2, -2, &x, 0, 0, 1);
    let output_graph = aligner.graph();
    println!("{:?}", Dot::new(&output_graph.map(|_, n| (*n) as char, |_, e| *e)));
    let mut all_paths: Vec<Vec<usize>> = vec![];
    let mut all_sequences: Vec<Vec<u8>> = vec![];

    // get topology ordering
    let mut topo = Topo::new(&output_graph);
    // go through the nodes topologically // make a hashmap with node_index as key and incrementing indices as value
    let mut topo_indices = vec![];
    let mut topo_map = HashMap::new();
    let mut incrementing_index: usize = 0;
    while let Some(node) = topo.next(&output_graph) {
        topo_indices.push(node.index());
        topo_map.insert(node.index(), incrementing_index);
        incrementing_index += 1;
    }
    simple_dfs_all_paths(output_graph, 0, vec![], vec![], &mut all_paths, &mut all_sequences, &topo_map);
    let (kmer_pos_vec, kmers_plus_k, kmer_path_vec, kmers_previous_node_in_paths) = find_kmer_matches(&y, &all_sequences, &all_paths, 2);
    lcskpp_graph(kmer_pos_vec, kmers_plus_k, kmer_path_vec, kmers_previous_node_in_paths, &all_paths, 2);
}