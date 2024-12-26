use std::rc::Rc;

use fxhash::FxHashMap;
use itertools::Itertools;
use petgraph::{algo::dijkstra, dot::Dot, graph::{DiGraph, NodeIndex}, visit::{EdgeRef, IntoNodeReferences, NodeFiltered, NodeRef, Reversed, Topo, Walker}, Direction, Graph};
use regex::Regex;

const INPUT: &str = include_str!("./input/day_24.txt");
// const INPUT: &str = r"x00: 1
// x01: 1
// x02: 1
// y00: 0
// y01: 1
// y02: 0

// x00 AND y00 -> z00
// x01 XOR y01 -> z01
// x02 OR y02 -> z02";
// const INPUT: &str = r"x00: 1
// x01: 0
// x02: 1
// x03: 1
// x04: 0
// y00: 1
// y01: 1
// y02: 1
// y03: 1
// y04: 1

// ntg XOR fgs -> mjb
// y02 OR x01 -> tnw
// kwq OR kpj -> z05
// x00 OR x03 -> fst
// tgd XOR rvg -> z01
// vdt OR tnw -> bfw
// bfw AND frj -> z10
// ffh OR nrd -> bqk
// y00 AND y03 -> djm
// y03 OR y00 -> psh
// bqk OR frj -> z08
// tnw OR fst -> frj
// gnj AND tgd -> z11
// bfw XOR mjb -> z00
// x03 OR x00 -> vdt
// gnj AND wpb -> z02
// x04 AND y00 -> kjc
// djm OR pbm -> qhw
// nrd AND vdt -> hwm
// kjc AND fst -> rvg
// y04 OR y02 -> fgs
// y01 AND x02 -> pbm
// ntg OR kjc -> kwq
// psh XOR fgs -> tgd
// qhw XOR tgd -> z09
// pbm OR djm -> kpj
// x03 XOR y03 -> ffh
// x00 XOR y04 -> ntg
// bfw OR bqk -> z06
// nrd XOR fgs -> wpb
// frj XOR qhw -> z04
// bqk OR frj -> z07
// y03 OR x01 -> nrd
// hwm AND bqk -> z03
// tgd XOR rvg -> z12
// tnw OR pbm -> gnj";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Gate {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Node {
    name: Rc<str>,
    gate: Option<Gate>,
}

impl Node {
    fn new(name: Rc<str>) -> Self {
        Self {
            name,
            gate: None,
        }
    }
}

fn build_graph() -> (DiGraph<Node, ()>, FxHashMap<Rc<str>, NodeIndex>) {
    let mut graph: DiGraph<Node, ()> = Default::default();
    let mut node_indices: FxHashMap<Rc<str>, NodeIndex> = Default::default();

    let gates_regex = Regex::new(r"(?<a>.{3}) (?<gate>XOR|OR|AND) (?<b>.{3}) -> (?<o>.{3})").unwrap();
    gates_regex.captures_iter(INPUT)
        .map(|captures| {
            let a: Rc<str> = Rc::from(captures.name("a").unwrap().as_str());
            let b: Rc<str> = Rc::from(captures.name("b").unwrap().as_str());
            let o: Rc<str> = Rc::from(captures.name("o").unwrap().as_str());
            let gate = match captures.name("gate").unwrap().as_str() {
                "AND" => Gate::And,
                "OR" => Gate::Or,
                "XOR" => Gate::Xor,
                _ => panic!("????"),
            };

            (a, b, o, gate)
        })
        .for_each(|(a, b, o, gate)| {
            let a = *node_indices.entry(a).or_insert_with_key(|a| graph.add_node(Node::new(a.clone())));
            let b = *node_indices.entry(b).or_insert_with_key(|b| graph.add_node(Node::new(b.clone())));
            let o = *node_indices.entry(o).or_insert_with_key(|o| graph.add_node(Node::new(o.clone())));

            let node = graph.node_weight_mut(o).unwrap();
            node.gate.replace(gate);

            graph.add_edge(a, o, ());
            graph.add_edge(b, o, ());
        });

    (graph, node_indices)
}

fn resolve_graph(graph: &DiGraph<Node, ()>, assignments: &mut FxHashMap<Rc<str>, bool>) {
    let filtered = NodeFiltered::from_fn(&graph, |node| graph.node_weight(node).is_some_and(|g| g.gate.is_some()));

    let mut visit = Topo::new(&filtered);
    let mut outputs: Vec<(Rc<str>, bool)> = Default::default();

    while let Some(n) = visit.next(&filtered) {
        let Some((a, b)) = graph.neighbors_directed(n, Direction::Incoming).next_tuple() else { continue };

        let Some(&a) = assignments.get(&graph.node_weight(a).unwrap().name) else { continue };
        let Some(&b) = assignments.get(&graph.node_weight(b).unwrap().name) else { continue };

        let Node { name,gate } = graph.node_weight(n).unwrap();
        let v = assignments.entry(name.clone()).or_default();
        match gate.as_ref().expect("Gate not initialized") {
            Gate::And => *v = a && b,
            Gate::Or =>  *v = a || b,
            Gate::Xor => *v = a != b,
        };
    }
}

fn evaluate_graph(graph: &DiGraph<Node, ()>, x: u64, y: u64) -> u64 {
    let mut assignments: FxHashMap<Rc<str>, bool> = (0..=44)
        .flat_map(|i: u64| {
            let x_set = (x >> i) & 1 == 1;
            let y_set = (y >> i) & 1 == 1;

            [(Rc::from(format!("x{i:02}")), x_set), (Rc::from(format!("y{i:02}")), y_set)]
        })
        .collect();
    
    resolve_graph(&graph, &mut assignments);
    get_output(&assignments)
}

fn get_output(assignments: &FxHashMap<Rc<str>, bool>) -> u64 {
    (0..=45u64).rev()
        .filter_map(|i| assignments.get(&Rc::from(format!("z{i:02}"))))
        .fold(0u64, |acc, &v| (acc << 1) | if v { 1 } else { 0 })
}

fn correctness(graph: &DiGraph<Node, ()>, x: u64, y: u64) -> usize {
    let output = evaluate_graph(graph, x, y);

    let expected_out = x + y;
    let incorrect_bits = expected_out ^ output;
    // println!("{incorrect_bits:048b}");
    (0..=45u64)
        .filter(|i| ((incorrect_bits >> i) & 1 == 1))
        .count()
}

fn swap_outputs(graph: &mut DiGraph<Node, ()>, a: NodeIndex, b: NodeIndex) -> bool {
    let Some(a_edge) = graph.edges_directed(a, Direction::Outgoing).next() else { return false };
    let Some(b_edge) = graph.edges_directed(b, Direction::Outgoing).next() else { return false };

    let a_target = a_edge.target();
    let b_target = b_edge.target();
    let a_id = a_edge.id();
    let b_id = b_edge.id();

    graph.remove_edge(a_id);
    graph.remove_edge(b_id);
    graph.add_edge(a, b_target, ());
    graph.add_edge(b, a_target, ());

    true
}

pub fn day_24() {
    println!("--- Day 24 ---");

    let (graph, node_indices) = build_graph();

    let assignments: FxHashMap<Rc<str>, bool> = INPUT.lines()
        .take_while(|l| l.len() > 0)
        .filter_map(|l| l.split(": ").next_tuple())
        .map(|(name, value)| {
            let name: Rc<str> = Rc::from(name);
            let value = value == "1";

            (name, value)
        })
        .collect();

    let output = {
        let mut assignments = assignments.clone();
        resolve_graph(&graph, &mut assignments);
        get_output(&assignments)
    };
    println!("Password: {output}");

    {
        let x = (0..=44u64).rev()
            .map(|i| assignments[&Rc::from(format!("x{i:02}"))])
            .fold(0u64, |acc, v| (acc << 1) | if v { 1 } else { 0 });
        let y = (0..=44u64).rev()
            .map(|i| assignments[&Rc::from(format!("y{i:02}"))])
            .fold(0u64, |acc, v| (acc << 1) | if v { 1 } else { 0 });

        let expected_out = x + y;
        let incorrect_bits = expected_out ^ output;
        println!("{incorrect_bits:048b}");

        // Rule 1: all outputs must be XORs (except z45)
        let non_xor_zs = graph.node_references()
            .filter(|(idx, node)| {
                node.name.starts_with("z")
                && node.gate != Some(Gate::Xor)
                && node.name.as_str() != "z45"
            })
            .collect::<Vec<_>>();
        println!("{non_xor_zs:?}");

        // Rule 2: all non-input gates must be AND/ORs
        let non_andor_xys = graph.node_references()
            .filter(|(idx, node)| {
                !node.name.starts_with("z")
                && node.gate == Some(Gate::Xor)
                && !graph.neighbors_directed(*idx, Direction::Incoming)
                    .all(|node| {
                        let name = &graph.node_weight(node).unwrap().name;
                        name.starts_with("x") || name.starts_with("y")
                    })
            })
            .collect::<Vec<_>>();
        println!("{non_andor_xys:?}");

        // Rule 3: all ANDs must lead into ORs
        let non_or_ands = graph.node_references()
            .filter(|(idx, node)| {
                node.gate == Some(Gate::And)
                && !graph.neighbors_directed(*idx, Direction::Outgoing)
                    .all(|node| {
                        let gate = graph.node_weight(node).unwrap().gate;
                        gate == Some(Gate::Or)
                    })
                && !graph.neighbors_directed(*idx, Direction::Incoming)
                    .all(|node| {
                        let name = &graph.node_weight(node).unwrap().name;
                        name.as_str() == "x00" || name.as_str() == "y00"
                    })
            })
            .collect::<Vec<_>>();
        println!("{non_or_ands:?}");

        // Rule 4: all XORs with inputs must lead into another XOR
        let non_xor_xors = graph.node_references()
            .filter(|(idx, node)| {
                node.gate == Some(Gate::Xor)
                && graph.neighbors_directed(*idx, Direction::Incoming)
                    .all(|node| {
                        let name = &graph.node_weight(node).unwrap().name;
                        name.starts_with("x") || name.starts_with("y")
                    })
                && !graph.neighbors_directed(*idx, Direction::Outgoing)
                    .any(|node| {
                        let gate = graph.node_weight(node).unwrap().gate;
                        gate == Some(Gate::Xor)
                    })
                && !graph.neighbors_directed(*idx, Direction::Incoming)
                    .all(|node| {
                        let name = &graph.node_weight(node).unwrap().name;
                        name.as_str() == "x00" || name.as_str() == "y00"
                    })
            })
            .collect::<Vec<_>>();
        println!("{non_xor_xors:?}");

        let mut all_incorrect = [non_xor_zs, non_andor_xys, non_or_ands, non_xor_xors].concat();
        // let swaps = all_incorrect.iter()
        //     .permutations(8)
        //     .find(|v| {
        //         let mut new_graph = graph.clone();
        //         v.into_iter()
        //             .array_chunks()
        //             .for_each(|[a, b]| {
        //                 swap_outputs(&mut new_graph, a.0, b.0);
        //             });
        //         correctness(&new_graph, x, y) == 0
        //     })
        //     .unwrap();
        all_incorrect.sort_by(|a, b| a.1.name.cmp(&b.1.name));
        let result = all_incorrect.into_iter()
            .map(|(_, node)| &node.name)
            .join(",");
        println!("Final swaps: {result}");
    }

    // println!("{:?}", Dot::new(&graph));
}