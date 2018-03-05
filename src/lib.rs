#![feature(inclusive_range_syntax)]

#[derive(Clone)]
struct Heap {
    pub size: usize,
    pub heap: Vec<i32>
}


impl Heap {
    pub fn new(heap: Vec<i32>) -> Heap {
        let mut base = vec![0];
        base.append(&mut heap.clone());
        let mut init = Heap {
            size: heap.len(),
            heap: base
        };
        init.build_heap();
        init
    }

    /// its parent is located at k/2 index
    pub fn node_parent(&self, i: usize) -> i32 {
        self.heap[i/2]
    }

    /// its right child is located at 2*k+1. index
    pub fn node_right(&self, i: usize) -> i32 {
        self.heap[i*2+1]
    }

    /// its left child is located at 2*k index
    pub fn node_left(&self, i: usize) -> i32 {
        self.heap[i*2]
    }

    pub fn lesser_child(&self, i: usize) -> (usize, i32 ) {
        if 2*i != self.size && self.node_left(i) > self.node_right(i) {
            (2*i+1, self.node_right(i))
        } else {
            (2*i, self.node_left(i))
        }
    }

    pub fn insert(&mut self, i: i32) {
        // resize the heap to fit new elements + tree structure
        if self.size == self.heap.len() - 1 {
            let len = self.heap.len();
            self.heap.resize(len * 2, 0x99999);
        }
        self.size += 1;
        let mut pos = self.size;

	      //Percolate up
        while pos > 1 && i < self.node_parent(pos) {
            self.heap[pos] = self.node_parent(pos);
            pos = pos/2;
        }

        self.heap[pos] = i;
    }

    pub fn percolate_down(&mut self, mut k: usize) {
        if self.heap.len() > 1 {
            let tmp = self.heap[k];
            while 2*k <= self.size {
                let (next_idx, lesser_child) = self.lesser_child(k);
                if tmp > lesser_child {
                    self.heap[k] = lesser_child;
                } else {
                    break;
                }
                k = next_idx;
            }
            self.heap[k] = tmp;
        }
    }

    /// DeleteMin
    ///    The minimum element can be found at the root, which is the first element of the array.
    /// We remove the root and replace it with the last element of the heap and then restore the
    /// heap property by percolating down. Similar to insertion,
    /// the worst-case runtime is O{log n).
    pub fn delete_min(&mut self) -> i32 {
        let out = self.heap.swap_remove(1);
        let len = self.heap.len();
        self.heap.resize(len + 1, 0x99999);
        // self.size -= 1;
        self.percolate_down(1);
        out
    }

    pub fn build_heap(&mut self) {
        for x in (1..=self.size/2).rev() {
            self.percolate_down(x)
        }
    }

    pub fn sort(&mut self) -> Vec<i32>{
        let mut dest = self.clone();
        let mut arr = Vec::new();
        for _ in (1..=dest.size).rev() {
            // println!("in heap: {:?}", dest.heap);
            arr.push(dest.delete_min());
            dest.build_heap();
        }
        arr
    }

    pub fn to_string(&self) -> String {
        let mut s = "".to_string();
        for x in self.heap.iter() {
            if *x != 1000 {
                s = format!("{} {}",s,x);
            }
        }
        s
    }
}


#[cfg(test)]
mod tests {

    use super::Heap;

    #[test]
    fn insert_value_sort() {
        let mut h = Heap::new(vec![]);
        for x in vec![8,3,4,9,4,3,-30,0,20000] {
            h.insert(x);
        }
        // h.build_heap();
        assert_eq!(h.sort(), vec![-30,0,3,3,4,4,8,9,20000]);
    }


    #[test]
    fn sort_heap() {
        let mut h = Heap::new(vec![8,3,4,9,4,3,0,0,12]);
        // h.build_heap();
        assert_eq!(h.sort(), vec![0,0,3,3,4,4,8,9,12]);
        // println!("build_heap {:?}", h.sort());
    }

    // TODO: the min-heap property: the value of each node is greater than or equal to the value of its parent, with the minimum-value element at the root.

}
