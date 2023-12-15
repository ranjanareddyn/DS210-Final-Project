
// importing all the necessary libraries 
use std::collections::{HashMap, HashSet, VecDeque};
use std::collections::hash_map::Entry;
use csv::ReaderBuilder;
use std::fs::File;
use std::io::Write;
use std::error::Error;
use std::env;
use std::path::Path;

#[derive(Debug)]
pub struct Graph {
    adj: HashMap<String, HashSet<String>>,
}

// the following defines a graph sturct with a single field 'adj'
// This structure is used to represent a directed graph where the key is a node and the value is a set of nodes to which it is connected.
impl Graph {
    // creating an empty graph
    pub fn new() -> Self {
        Graph { adj: HashMap::new() }
    }

    // this part adds a node with a string label and if the node already exists it should do nothing 

    pub fn add_node(&mut self, u: String) {
        self.adj.entry(u).or_insert(HashSet::new());
    }

    // generating DOT representation of the graph, this is used for visualizing the graph 
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph G {\n");
        for (node, edges) in &self.adj {
            for edge in edges {
                dot.push_str(&format!("    \"{}\" -> \"{}\";\n", node, edge));
            }
        }
        dot.push_str("}\n");
        dot
    }

    // the next part adds a directed edge from u to v (u->v)
    fn add_directed_edge(&mut self, u: String, v: String) {
        match self.adj.entry(u) {
            Entry::Occupied(succ) => {
                succ.into_mut().insert(v);
            }
            Entry::Vacant(succ) => {
                succ.insert(HashSet::from([v]));
            }
        }
    }

    // Adding an edge in the graph (u<->v)
    pub fn add_edge(&mut self, u: String, v: String) {
        self.add_directed_edge(u.clone(), v.clone());
        self.add_directed_edge(v, u);
    }

    //the following returns a set of all nodes present in the graph 
    pub fn nodes(&self) -> HashSet<String> {
        self.adj.keys().cloned().collect()
    }

    // this part returns the adjacent nodes of any given node 'u'
    pub fn adj(&self, u: &String) -> Option<&HashSet<String>> {
        self.adj.get(u)
    }

    // computing the degrees of separation between two nodes and they are considered the shortest path between two nodes in a graph 
    pub fn degrees_of_separation(&self, start: &String, end: &String) -> Option<usize> {
        if start == end {
            return Some(0);
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start.clone(), 0));

        while let Some((current, distance)) = queue.pop_front() {
            if current == *end {
                return Some(distance);
            }

            if !visited.insert(current.clone()) {
                continue;
            }

            if let Some(neighbors) = self.adj.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        queue.push_back((neighbor.clone(), distance + 1));
                    }
                }
            }
        }
        None
    }

    // Method to add nodes and edges from CSV data
    pub fn add_from_csv(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(filename)?;
        let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

        for result in reader.records() {
            let record = result?;
            let name = record.get(0).unwrap().to_string();
            self.add_node(name);
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut graph = Graph::new();

    // Add selected subreddit names
    let askreddit = "askreddit".to_string();
    let globaloffensivetrade = "globaloffensivetrade".to_string();
    let fireteams = "fireteams".to_string();
    let funny = "funny".to_string();
    let the_donald = "the_donald".to_string();

    // Add selected user names
    let rotoreuters = "rotoreuters".to_string();
    let fiplefip = "fiplefip".to_string();
    let amici_ursi = "amici_ursi".to_string();
    let unremovable = "unremovable".to_string();
    let cdre_64 = "CDRE_64".to_string();

    // Add nodes to the graph
    graph.add_node(askreddit.clone());
    graph.add_node(globaloffensivetrade.clone());
    graph.add_node(fireteams.clone());
    graph.add_node(funny.clone());
    graph.add_node(the_donald.clone());
    graph.add_node(rotoreuters.clone());
    graph.add_node(fiplefip.clone());
    graph.add_node(amici_ursi.clone());
    graph.add_node(unremovable.clone());
    graph.add_node(cdre_64.clone());

    // Create example edges between subreddits and users
    // These are hypothetical and should be based on your data
    graph.add_edge(askreddit.clone(), rotoreuters.clone());
    graph.add_edge(globaloffensivetrade.clone(), fiplefip.clone());
    graph.add_edge(fireteams.clone(), amici_ursi.clone());
    graph.add_edge(funny.clone(), unremovable.clone());
    graph.add_edge(the_donald.clone(), cdre_64.clone());


    // Load data from CSV files (Assuming they contain embedding data)
    graph.add_from_csv("/Users/ranjanareddy/Desktop/web-redditEmbeddings-users.csv")?;
    graph.add_from_csv("/Users/ranjanareddy/Desktop/web-redditEmbeddings-subreddits.csv")?;

    // Update these paths to the correct locations of your CSV files
    let users_csv_path = "/Users/ranjanareddy/Desktop/web-redditEmbeddings-users.csv";
    let subreddits_csv_path = "/Users/ranjanareddy/Desktop/web-redditEmbeddings-subreddits.csv";

    graph.add_from_csv(users_csv_path);
    graph.add_from_csv(subreddits_csv_path);

    // Print the graph nodes (subreddits, users, and interactions)
    println!("{:?}", graph.nodes());

    // // computing degrees of separation between two nodes
    // let start = "rotoreuters".to_string();
    // let end = "askreddit".to_string();
    // match graph.degrees_of_separation(&start, &end) {
    //     Some(degrees) => println!("Degrees of separation between {} and {}: {}", start, end, degrees),
    //     None => println!("No path found between {} and {}", start, end),
    // }


    let nodes = vec!["rotoreuters", "askreddit", "fireteams", "funny"]; // example nodes

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let start = nodes[i].to_string();
            let end = nodes[j].to_string();

            match graph.degrees_of_separation(&start, &end) {
                Some(degrees) => println!("Degrees of separation between {} and {}: {}", start, end, degrees),
                None => println!("No path found between {} and {}", start, end),
            }
        }
    }

    // Generate DOT format output
    let dot_output = graph.to_dot();

    // Save the DOT output to a file
    let mut file = File::create("graph.dot")?;
    file.write_all(dot_output.as_bytes())?;

    println!("Graph saved to graph.dot");

    let dot_output = graph.to_dot();
    println!("{}", dot_output);

    Ok(())
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph() {
        let g = Graph::new();
        assert_eq!(g.nodes(), HashSet::new());
    }
}




