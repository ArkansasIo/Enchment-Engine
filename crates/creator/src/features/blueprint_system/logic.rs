    /// Execute the blueprint graph starting from a given node index.
    pub fn execute(&self, start: usize) {
        let mut visited = vec![false; self.nodes.len()];
        let mut stack = vec![start];

        while let Some(idx) = stack.pop() {
            if idx >= self.nodes.len() || visited[idx] {
                continue;
            }
            visited[idx] = true;
            let node = &self.nodes[idx];
            // Placeholder: handle each node type
            match node {
                BlueprintNode::Event(name) => {
                    println!("Executing Event node: {}", name);
                }
                BlueprintNode::Function(name) => {
                    println!("Executing Function node: {}", name);
                }
                BlueprintNode::Variable(name) => {
                    println!("Accessing Variable node: {}", name);
                }
                BlueprintNode::Branch => {
                    println!("Executing Branch node");
                }
                BlueprintNode::Sequence => {
                    println!("Executing Sequence node");
                }
                BlueprintNode::Quest { name } => {
                    println!("Executing Quest node: {}", name);
                }
                BlueprintNode::Inventory => {
                    println!("Executing Inventory node");
                }
                BlueprintNode::Dialogue { npc } => {
                    println!("Executing Dialogue node: {}", npc);
                }
                BlueprintNode::Combat => {
                    println!("Executing Combat node");
                }
                BlueprintNode::Guild => {
                    println!("Executing Guild node");
                }
                BlueprintNode::Trade => {
                    println!("Executing Trade node");
                }
                BlueprintNode::Party => {
                    println!("Executing Party node");
                }
                BlueprintNode::PvP => {
                    println!("Executing PvP node");
                }
                BlueprintNode::WorldEvent(name) => {
                    println!("Executing WorldEvent node: {}", name);
                }
                BlueprintNode::Custom(name) => {
                    println!("Executing Custom node: {}", name);
                }
            }

            // Traverse to connected nodes
            for &(_, to) in self.connections.iter().filter(|(from, _)| *from == idx) {
                stack.push(to);
            }
        }
    }
//! Blueprint Logic Engine

use super::nodes::BlueprintNode;

pub struct BlueprintGraph {
    pub nodes: Vec<BlueprintNode>,
    pub connections: Vec<(usize, usize)>, // (from, to)
}

impl BlueprintGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: BlueprintNode) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    pub fn connect(&mut self, from: usize, to: usize) {
        self.connections.push((from, to));
    }
}
