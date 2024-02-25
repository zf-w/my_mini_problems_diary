pub struct UnionFind {
    v: Vec<usize>,
}

impl UnionFind {
    pub fn new_with_size(size: usize) -> Self {
        let mut v = Vec::with_capacity(size);
        for i in 0..size {
            v.push(i);
        }
        Self { v }
    }

    pub fn find(&mut self, i: usize) -> usize {
        let prev = self.v[i];
        if prev == i {
            return i;
        }
        let prev = self.find(prev);
        self.v[i] = prev.clone();
        prev
    }

    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let prev_i = self.find(i);
        let prev_j = self.find(j);

        if prev_i == prev_j {
            return false;
        }

        self.v[prev_j] = prev_i.clone();
        true
    }
}
