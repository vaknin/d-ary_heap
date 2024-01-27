use core::panic;
use dialoguer::Input;

pub struct Dheap {
    pub d: usize,
    pub nodes: Vec<i32>
}

impl Dheap {
    /// Creates a new D-ary heap with specified branching factor and nodes.
    ///
    /// Initializes the heap with a given branching factor (d) and a vector of nodes.
    /// Builds a max-heap from the provided nodes.
    pub fn new(d: usize, nodes: Vec<i32>) -> Self {
        let mut heap = Dheap { d, nodes };
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
        let non_leaf_nodes = (self.nodes.len() as f32 / self.d as f32).ceil() as usize;
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
    pub fn increase_key(&mut self, mut i: usize, key: i32, quiet: bool) {

        // Check if the index exists and if the new key is smaller than the current key
        match self.nodes.get(i) {
            Some(_) => {
                if key < self.nodes[i] {
                    return eprintln!("New key is smaller than current key!");
                }
            },
            None => { return eprintln!("The index '{i}' doesn't exist in the heap!") }
        }

        if !quiet { println!("Increased index [{}] from [{}] to [{}]", i, self.nodes[i], key) }
        self.nodes[i] = key;

        // Loop to adjust the heap: move the node up the tree as long as
        // it is larger than its parent
        while i > 0 && self.nodes[i] > self.nodes[self.parent_of(i)] {

            // Swap the current node with its parent
            let parent = self.parent_of(i);
            self.nodes.swap(i, parent);

            // Update the index to that of the parent for the next iteration
            i = parent;
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
        println!("Inserted: [{key}]");
        self.increase_key(self.nodes.len() - 1, key, true);
    }

    /// Deletes a node from the max-heap at a given index.
    ///
    /// Prompts the user to enter the index of the node to be deleted. Replaces the node at this
    /// index with the maximum integer value, moves it to the top, and then removes it using Extract-Max
    pub fn delete(&mut self) {
        // Prompt the user
        let i: usize = Input::new()
            .with_prompt("Enter the index to delete")
            .interact_text()
            .unwrap();
        if self.nodes.get(i).is_none() {
            return eprintln!("The index '{i}' doesn't exist in the heap!")
        }
        
        println!("Deleted index [{i}] with value [{}]", self.nodes[i]);

        // Replace the node with infinity, thus bringing him to the top of the node
        self.increase_key(i, i32::MAX, true);

        // apply 'Extract Max' to delete the top node
        self.extract_max();

    }

    /// Prints the heap in a structured format.
    ///
    /// Outputs each level of the heap in a parenthesized, comma-separated format.
    /// The nodes at each level are enclosed in parentheses and separated by commas.
    pub fn print(&self) {
        let mut level_start = 0; // Start index of the current level
        let mut nodes_at_level = 1; // Number of nodes at the current level, starts with 1 for the root

        while level_start < self.nodes.len() {
            let level_end = std::cmp::min(level_start + nodes_at_level, self.nodes.len()); // End index of the current level

            // Print the nodes at the current level
            print!("(");
            for i in level_start..level_end {
                print!("{}", self.nodes[i]);
                if i < level_end - 1 {
                    print!(","); // Separate nodes with a comma
                }
            }
            print!(")");

            if level_end < self.nodes.len() {
                println!(); // New line after each level, except the last
            }

            // Update for the next level
            level_start = level_end; // Move to the next level
            nodes_at_level *= self.d; // Increase the number of nodes at the next level
        }
        println!(); // New line at the end for clean separation
    }
}