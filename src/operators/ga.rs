/// This Source Code Form is subject to the terms of The GNU General Public License v3.0
/// Copyright 2024 - Guilherme Santos. If a copy of the GPL3 was not distributed with this
/// file, You can obtain one at https://www.gnu.org/licenses/gpl-3.0.html
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use rand::prelude::*;

use crate::operators::Partition;

/// Generates the initial population of random partitions.
/// Generates the initial population of random partitions with controlled community sizes.
pub fn generate_initial_population(
    graph: &Graph<(), (), Undirected>,
    population_size: usize,
) -> Vec<Partition> {
    let mut rng = thread_rng();
    let nodes: Vec<NodeIndex> = graph.node_indices().collect();
    let num_nodes = nodes.len();
    let max_initial_communities = (num_nodes as f64).sqrt() as usize; // Example heuristic
    let min_initial_communities = 2;
    let mut population = Vec::with_capacity(population_size);

    for _ in 0..population_size {
        // Randomly decide the number of communities for this partition
        let num_communities = rng.gen_range(min_initial_communities..=max_initial_communities);
        
        // Shuffle nodes to assign them randomly to communities
        let mut shuffled_nodes = nodes.clone();
        shuffled_nodes.shuffle(&mut rng);
        
        let mut partition = Partition::default();
        for (idx, node) in shuffled_nodes.iter().enumerate() {
            let community_id = idx % num_communities;
            partition.insert(*node, community_id as usize);
        }
        population.push(partition);
    }

    population
}

/// Performs two-point crossover between two parents.
pub fn crossover(parent1: &Partition, parent2: &Partition) -> Partition {
    let mut rng = thread_rng();
    let keys: Vec<&NodeIndex> = parent1.keys().collect();
    let mut child = Partition::default();

    for &node in keys {
        // With 50% probability, inherit the community from parent1 or parent2
        let inherit_from_parent2 = rng.gen_bool(0.5);
        let community = if inherit_from_parent2 {
            parent2[&node]
        } else {
            parent1[&node]
        };
        child.insert(node, community);
    }

    child
}

/// Mutates a partition by changing a node's community to that of a neighbor.
pub fn mutate(partition: &mut Partition, graph: &Graph<(), (), Undirected>) {
    let mut rng = thread_rng();
    let nodes: Vec<&NodeIndex> = partition.keys().collect();
    if nodes.is_empty() {
        return;
    }

    let node = nodes.choose(&mut rng).unwrap();
    let neighbors: Vec<NodeIndex> = graph.neighbors(**node).collect();
    if !neighbors.is_empty() {
        let neighbor = neighbors.choose(&mut rng).unwrap();
        partition.insert(**node, *partition.get(neighbor).unwrap());
    }
}

/// Selects the top half of the population based on modularity.
pub fn selection(population: &[Partition], fitnesses: &[(f64, f64, f64)]) -> Vec<Partition> {
    let mut pop_fitness: Vec<(&Partition, f64)> = population
        .iter()
        .zip(fitnesses.iter().map(|f| f.0))
        .collect();

    // Sort by modularity descending
    pop_fitness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // Select top half
    pop_fitness
        .iter()
        .take(population.len() / 2)
        .map(|&(partition, _)| partition.clone())
        .collect()
}
