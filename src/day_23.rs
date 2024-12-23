use std::iter::once;

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use petgraph::{graph::{NodeIndex, UnGraph}, Directed, Graph, Undirected};

const INPUT: &str = include_str!("./input/day_23.txt");
// const INPUT: &str = r"kh-tc
// qp-kh
// de-cg
// ka-co
// yn-aq
// qp-ub
// cg-tb
// vc-aq
// tb-ka
// wh-tc
// yn-cg
// kh-ub
// ta-co
// de-co
// tc-td
// tb-wq
// wh-td
// ta-ka
// td-qp
// aq-cg
// wq-ub
// ub-vc
// de-ta
// wq-aq
// wq-vc
// wh-yn
// ka-de
// kh-ta
// co-tc
// wh-qp
// tb-vc
// td-yn";

type Computer = [char; 2];
type Network = UnGraph<Computer, ()>;

type NodeSet = FxHashSet<NodeIndex>;
pub fn maximal_cliques(graph: &Network, clique: NodeSet, mut candidates: NodeSet, mut excluded: NodeSet) -> Box<dyn Iterator<Item = NodeSet> + '_> {
    if candidates.len() == 0 && excluded.len() == 0 {
        Box::new(once(clique.into_iter().collect()))
    } else {
        Box::new(std::iter::from_fn(move || {
            if let Some(&v) = candidates.iter().next() {
                let mut new_clique = clique.clone();
                new_clique.insert(v);

                let neighbour_set: NodeSet = graph.neighbors(v).collect();
                let max_cliques = maximal_cliques(graph, new_clique, candidates.intersection(&neighbour_set).copied().collect(), excluded.intersection(&neighbour_set).copied().collect());
                candidates.remove(&v);
                excluded.insert(v);
                Some(max_cliques)
            } else { None }
        }).flatten())
    }
}

pub fn all_maximal_cliques(graph: &Network) -> Box<dyn Iterator<Item = NodeSet> + '_> {
    let candidates = graph.node_indices().collect();
    maximal_cliques(graph, Default::default(), candidates, Default::default())
}

pub fn day_23() {
    println!("--- Day 23 ---");

    let mut network = Network::default();
    let mut node_indices = FxHashMap::default();
    INPUT
        .lines()
        .map(|line| {
            line.split("-")
                .map(|s| -> Computer {
                    s.chars().next_chunk().unwrap()
                })
                .next_tuple::<(Computer, Computer)>()
                .unwrap()
        })
        .for_each(|(start, end)| {
            let s = *node_indices.entry(start).or_insert_with(|| network.add_node(start));
            let e = *node_indices.entry(end).or_insert_with(|| network.add_node(end));

            network.add_edge(s, e, ());
        });

    let subgraph: Graph<(), (), Undirected, usize> = Graph::from_edges(&[(0, 1), (1, 2), (2, 0)]);

    let num_cycles = petgraph::algo::subgraph_isomorphisms_iter(
        &&subgraph,
        &&network,
        &mut |_, _| true,
        &mut |_, _| true,
    ).unwrap()
        .map(|mut computers| {
            computers.sort();
            computers.into_iter()
                .map(|n| network[NodeIndex::new(n)])
                .next_chunk::<3>()
                .unwrap()
        })
        .unique()
        .filter(|computers| computers.iter().any(|c| c[0] == 't'))
        .count();
    println!("Number of matching triples: {num_cycles}");

    let maximum_clique = all_maximal_cliques(&network)
        // .inspect(|clique| {
        //     println!("{clique:?}");
        // })
        .max_by_key(|clique| clique.len())
        .unwrap();
    println!("Maximum clique: {maximum_clique:?}");
    let mut computers = maximum_clique.into_iter().map(|node| network.node_weight(node).unwrap()).collect::<Vec<_>>();
    computers.sort_by(|a, b| {
        match a[0].cmp(&b[0]) {
            std::cmp::Ordering::Equal => a[1].cmp(&b[1]),
            other => other,
        }
    });
    let password = computers.into_iter()
        .map(|computer| computer.into_iter().collect::<String>())
        .join(",");
    println!("Password: {password}");
}