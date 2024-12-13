use petgraph::graph::Graph;
use petgraph::Undirected;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use petgraph::graph::UnGraph;
use std::io::{self, BufRead};
use std::path::Path;

mod algorithm;

const NUM_GENERATIONS: usize = 800;
const POPULATION_SIZE: usize = 100;

fn read_graph(file_path: &str) -> UnGraph<(), ()> {
    let mut graph = Graph::new_undirected();

    let mut node_indices = std::collections::HashMap::new();

    // Open the file and iterate over its lines
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(entry) = line {
                // Parse the line into source and target
                let parts: Vec<&str> = entry.split(',').collect();
                if parts.len() >= 2 {
                    let source = parts[0].parse::<usize>().unwrap();
                    let target = parts[1].parse::<usize>().unwrap();

                    // Add nodes to the graph if they don't exist
                    let src_index = *node_indices.entry(source).or_insert_with(|| graph.add_node(()));
                    let tgt_index = *node_indices.entry(target).or_insert_with(|| graph.add_node(()));

                    // Add an edge between the nodes
                    graph.add_edge(src_index, tgt_index, ());
                }
            }
        }
    }
    graph
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn print_graph(graph : Graph<(), (), Undirected>){
    for edge in graph.edge_indices() {
        let (source, target) = graph.edge_endpoints(edge).unwrap();
        println!("Edge: {} -> {}", source.index(), target.index());
    }
}

fn main() {
    let mut file_path = "/home/ol1ve1r4/Desktop/mocd/rust/src/mu-005.edgelist";
    let mut data: Vec<String> = Vec::new();
    let graph: Graph<(), (), Undirected> = 
    read_graph(&file_path);
        
    let start: Instant = std::time::Instant::now();

    let (
        best_partition,
        deviations,
        real_fitnesses,
        random_fitnesses,
        best_history,
        avg_history,
    ) = algorithm::genetic_algorithm(&graph, NUM_GENERATIONS, POPULATION_SIZE);

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

        // Save in DataFrame
    for generation in 0..NUM_GENERATIONS {
        data.push(format!(
            "{},{},{}",
            generation,
            best_history[generation],
            avg_history[generation],
        ));
    }
    
    let mut file = File::create("generations_data.csv").expect("Unable to create file");
    writeln!(
        file,
        "iteration,nodes_num,tau1,tau2,mu,min_community,max_community,generation,best_history,avg_history,nmi_score"
    )
    .unwrap();
    for line in data {
        writeln!(file, "{}", line).unwrap();
    }
    println!("Data saved to generations_data.csv");
}