pub struct Dheap {
    pub d: usize,
    pub nodes: Vec<i32>,
    pub height: usize
}

impl Dheap {
    pub fn new(d: usize, nodes: Vec<i32>) -> Self {
        let height = (((nodes.len() * (d - 1) + 1) as f32).log(d as f32).ceil() - 1.0) as usize;
        Dheap {
            d,
            nodes,
            height
        }
    }
}

// Make the d-ary heap printable
impl std::fmt::Display for Dheap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut level_start = 0;
        let mut nodes_at_level = 1;

        while level_start < self.nodes.len() {
            let level_end = std::cmp::min(level_start + nodes_at_level, self.nodes.len());

            write!(f, "(")?;
            for i in level_start..level_end {
                if i > level_start {
                    write!(f, ",")?;
                }
                write!(f, "{}", self.nodes[i])?;
            }
            write!(f, ")")?;

            if level_end < self.nodes.len() {
                write!(f, " ")?;
            }

            level_start = level_end;
            nodes_at_level *= self.d;
        }

        Ok(())
    }
}