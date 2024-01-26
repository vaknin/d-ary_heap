use core::panic;
use std::cmp::min;

use dialoguer::Input;

pub struct Dheap {
    pub d: usize,
    pub nodes: Vec<i32>,
    pub height: usize
}

impl Dheap {
    /// Creates a new D-ary heap with specified branching factor and nodes.
    ///
    /// Initializes the heap with a given branching factor (d) and a vector of nodes.
    /// Calculates the height of the heap and then builds a max-heap from the provided nodes.
    pub fn new(d: usize, nodes: Vec<i32>) -> Self {
        let height = (((nodes.len() * (d - 1) + 1) as f32).log(d as f32).ceil() - 1.0) as usize;
        let mut heap = Dheap { d, nodes, height };
        heap.build_max_heap();
        heap
    }

    /// Calculates the parent index of a given node index in the D-ary heap.
    ///
    /// Computes the parent index based on the heap's branching factor. 
    /// Panics if the index is 0, as the first node has no parent.
    fn parent_of(&self, i: usize) -> usize {
        if i == 0 {
            panic!("Cannot calculate parent for the first node!");
        }
        let mut value = i as f32 / self.d as f32;
        value = if i % self.d < self.d { value.ceil() } else { value.floor() };
        value as usize - 1
    }

    /// Determines the nth child index of a given node index in the D-ary heap.
    ///
    /// Calculates the index of the nth child for a node at index 'i', 
    /// based on the heap's branching factor.
    fn nth_child_of(&self, n: usize, i: usize) -> usize {
        self.d * (i + 1) - self.d + n + 1
    }

    /// Constructs a max-heap from an unordered array.
    ///
    /// Iterates over each non-leaf node in reverse order and applies `max_heapify` to each,
    /// building a valid max-heap from the bottom up.
    pub fn build_max_heap(&mut self) {
        let non_leaf_nodes = (self.nodes.len() as f32 / self.d as f32).floor() as usize; // there are floor(n/d) non-leaf nodes
        for i in (0..non_leaf_nodes).rev() {
            self.max_heapify(i);
        }
    }

    /// Maintains the max-heap property for a node at a given index.
    ///
    /// Compares the node with its children and swaps it with the largest child if necessary.
    /// Recursively applies this process to ensure the subtree rooted at the index is a max-heap.
    fn max_heapify(&mut self, i: usize) {

        let mut largest = i;
    
        // Iterate through all children of the node
        for n in 0..self.nodes.len() {
            // Calculate the index of the nth child of the current node
            let nth_child = self.nth_child_of(n, i);
    
            // Break the loop if the child index is beyond the length of the nodes array
            if nth_child >= self.nodes.len() {
                break;
            }
    
            // Update the largest value if the nth child is greater than the current largest
            if self.nodes[nth_child] > self.nodes[largest] {
                largest = nth_child;
            }
        }
    
        // Check if the largest is not the current node
        if largest != i {
            // Swap the current node with the largest node
            self.nodes.swap(i, largest);
    
            // Recursively heapify the affected sub-tree
            self.max_heapify(largest);
        }
    }

    /// Removes and returns the maximum element from the max-heap.
    ///
    /// Swaps the first element (largest) with the last, removes the last element, 
    /// and then applies `max_heapify` to the new root to maintain the max-heap property.
    /// Returns the maximum element.
    pub fn extract_max(&mut self) -> i32 {
        let largest = self.nodes[0];
        self.nodes[0] = self.nodes[self.nodes.len() - 1];
        self.nodes.pop();
        self.max_heapify(0);
        largest
    }

    /// Increases the key of a node at a specified index in the max-heap.
    ///
    /// Validates the existence of the index and ensures the new key is greater than the current key.
    /// Updates the key and restructures the heap to maintain the max-heap property.
    /// Prints an error message if the index does not exist or the new key is smaller than the current key.
    pub fn increase_key(&mut self, mut i: usize, key: i32) {

        // Check if the index exists and if the new key is smaller than the current key
        match self.nodes.get(i) {
            Some(_) => {
                if key < self.nodes[i] {
                    return eprintln!("New key is smaller than current key!");
                }
            },
            None => { return eprintln!("The index '{i}' doesn't exist in the heap!") }
        }

        self.nodes[i] = key;

        // If the node is the root, return early
        // as no further action is needed and parent cannot be calculated
        if i == 0 { return }

        let i_parent = self.parent_of(i);

        // Loop to adjust the heap: move the node up the tree as long as
        // it is larger than its parent
        while i > 0 && self.nodes[i_parent] < self.nodes[i] {
            // Swap the current node with its parent
            self.nodes.swap(i, i_parent);

            // Update the index to that of the parent for the next iteration
            i = i_parent;
        }
    }

    /// Inserts a new node into the max-heap.
    ///
    /// Prompts the user for a key to insert. Adds a new node with the minimum integer value (negative infinity) 
    /// and then increases its key to the user-provided value, maintaining the max-heap property.
    pub fn insert(&mut self) {
        let key: i32 = Input::new()
            .with_prompt("Enter the key to insert")
            .interact_text()
            .unwrap();
        self.nodes.push(i32::MIN); // equivalent to adding negative infinity
        self.increase_key(self.nodes.len() - 1, key);
    }

    /// Deletes a node from the max-heap at a given index.
    ///
    /// Prompts the user to enter the index of the node to be deleted. Replaces the node at this
    /// index with the maximum integer value, moves it to the top, and then removes it. Ensures
    /// the heap maintains max-heap property after deletion.
    pub fn delete(&mut self) {
        // Prompt the user
        let i: usize = Input::new()
            .with_prompt("Enter the index to delete")
            .interact_text()
            .unwrap();
        if self.nodes.get(i).is_none() {
            return eprintln!("The index '{i}' doesn't exist in the heap!")
        }
        
        // Replace the node with infinity, thus bringing him to the top of the node
        self.increase_key(i, i32::MAX);

        // Now, let's swap the first and the last nodes
        let len = self.nodes.len();
        self.nodes.swap(0, len - 1);

        // Now, the key we wanted to remove is the last node, we'll delete it
        self.nodes.pop();

        // And make sure we maintain the max-heap property
        self.max_heapify(0);
    }
}

// Make the d-ary heap printable
impl std::fmt::Display for Dheap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut level_start = 0;
        let mut nodes_at_level = 1;

        while level_start < self.nodes.len() {
            let level_end = min(level_start + nodes_at_level, self.nodes.len());

            write!(f, "(")?;
            for i in level_start..level_end {
                if i > level_start {
                    write!(f, ",")?;
                }
                write!(f, "{}", self.nodes[i])?;
            }
            write!(f, ")")?;

            if level_end < self.nodes.len() {
                writeln!(f)?;
            }

            level_start = level_end;
            nodes_at_level *= self.d;
        }

        Ok(())
    }
}
