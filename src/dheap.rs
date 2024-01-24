struct Dheap {
    pub d: u32,
    pub nodes: Vec<i32>
}

impl Dheap {
    pub fn new(d: u32, nodes: Vec<i32>) -> Self {
        Dheap { d, nodes }
    }
}