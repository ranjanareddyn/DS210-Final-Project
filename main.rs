
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
// this structure is used to represent a directed graph where the key is a node and the value is a set of nodes to which it is connected

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

    // adding an edge in the graph (u<->v)
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

        // the next part initializes a Hashset that will store nodes that have already been visited during the search process and mut queue performs analysis using Breath-first search (BFS) which is generally used to find the shortest path beetween two nodes
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((start.clone(), 0));

        //we now begins a loop that continues as long as there are elements in the queue 
        while let Some((current, distance)) = queue.pop_front() {

            //if current == *end checks if the current node is the destination node, and if so returns the distence to this node 
            if current == *end { 
                return Some(distance);
            }

            //if the node is already in the set (meaning it has been visited before), the insert method returns false and the loop continues to the next iteration, skipping the already visited node
            if !visited.insert(current.clone()) {
                continue;
            }

            //retrieves the set of nodes that are adjacent to the current node 
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

    // the following is a method to add nodes and edges from CSV data
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

    // Adding selected subreddit names
    let askreddit = "askreddit".to_string();
    let globaloffensivetrade = "globaloffensivetrade".to_string();
    let fireteams = "fireteams".to_string();
    let funny = "funny".to_string();
    let the_donald = "the_donald".to_string();

    // Adding selected user names
    let rotoreuters = "rotoreuters".to_string();
    let fiplefip = "fiplefip".to_string();
    let amici_ursi = "amici_ursi".to_string();
    let unremovable = "unremovable".to_string();
    let cdre_64 = "CDRE_64".to_string();

    // Adding nodes to the graph
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

    // Creating edges between subreddits and users
    graph.add_edge(askreddit.clone(), rotoreuters.clone());
    graph.add_edge(globaloffensivetrade.clone(), fiplefip.clone());
    graph.add_edge(fireteams.clone(), amici_ursi.clone());
    graph.add_edge(funny.clone(), unremovable.clone());
    graph.add_edge(the_donald.clone(), cdre_64.clone());


    // Loading data from CSV files 
    graph.add_from_csv("/Users/ranjanareddy/Desktop/web-redditEmbeddings-users.csv")?;
    graph.add_from_csv("/Users/ranjanareddy/Desktop/web-redditEmbeddings-subreddits.csv")?;

    let users_csv_path = "/Users/ranjanareddy/Desktop/web-redditEmbeddings-users.csv";
    let subreddits_csv_path = "/Users/ranjanareddy/Desktop/web-redditEmbeddings-subreddits.csv";

    graph.add_from_csv(users_csv_path);
    graph.add_from_csv(subreddits_csv_path);

    // Print the graph nodes (subreddits, users, and interactions)
    println!("{:?}", graph.nodes());


    // This part creates a vector and each string represnts the name of a node in the graph
    let nodes = vec!["rotoreuters", "askreddit", "fireteams", "funny"]; 

    // This line starts an outer loop that iterates over the indices of the nodes vector
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {

            // this part accesses the node at index i in the nodes vector and converts it to a string and this is the starting node 
            let start = nodes[i].to_string();
            // this part accesses the node at index j in the nodes vector and converts it to a string and this is the ending node 
            let end = nodes[j].to_string();

            // this method shows returns the shortest path between the start and end nodes, given that a path exissts 
            match graph.degrees_of_separation(&start, &end) {
                Some(degrees) => println!("Degrees of separation between {} and {}: {}", start, end, degrees),
                None => println!("No path found between {} and {}", start, end),
            }
        }
    }

    // Generating DOT format output
    let dot_output = graph.to_dot();

    // Saving the DOT output to a file
    let mut file = File::create("graph.dot")?;
    file.write_all(dot_output.as_bytes())?;

    println!("Graph saved to graph.dot");

    let dot_output = graph.to_dot();
    println!("{}", dot_output);

    Ok(())
}

// Testing the code 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph() {
        let g = Graph::new();
        assert_eq!(g.nodes(), HashSet::new());
    }
}
