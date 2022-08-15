pub struct Search {
    root : Node,
}

pub struct Node {
    children : Vec<Node>,
    num_visits : u64,
    board : SmallBoard,
    cumulative_score : f64,
}

impl Search {
    pub fn new(starting_board : &SmallBoard) -> Search{
        Search {
            root : Node {
                children : vec![],
                num_visits : 0,
                board : starting_board,
                cumulative_score : 0.0,
            }
        }
    }

    pub fn iterate(&mut self) {

    }
}


impl Node {
    fn best_child(&self) -> usize {
        todo!()
    }


    fn iterate(&mut self) -> f64 {
        // if this node is unexpanded:
        if self.children.len() == 0 {
            // create all the children from this state
            self.populate();
            // pick a random child
            let mut node = self.nodes[self.pick_random()];
            // find the value of the node
            let value = node.value();
            // add the score to the cumulative value of the chosen child
            node.cumulative_score = value;
            // add the flipped version of that score to the current node
            self.cumulative_score += 1 - value;
            // increment my total visits
            self.num_visits += 1;
            // increment the child's visits
            node.num_vists += 1;
            return value;
        }
        
        // find the best child from my perspective
        let next_node = node.best_child();
        let value = 1 - Self::actually_iterate(&mut node.children[next_node]);
        self.cumulative_score += value;
        self.num_visits += 1;
        value
    }
}