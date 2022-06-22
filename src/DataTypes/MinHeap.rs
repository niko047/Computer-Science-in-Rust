
/* MinHeap, definition:
A binary heap is a heap data structure that takes the form of a binary tree.
A binary heap is defined as a binary tree with two additional constraints:

Shape property: a binary heap is a complete binary tree; that is, all levels of the tree,
except possibly the last one (deepest) are fully filled, and, if the last level of the tree is not complete,
the nodes of that level are filled from left to right.
Heap property: the key stored in each node is either greater than or equal to (≥) or less than or equal to (≤)
the keys in the node's children, according to some total order.
*/
#[derive(Debug)]
struct MinHeap<T> (Vec<T>);

impl<T> MinHeap<T>
    where T : PartialOrd + Copy {
    fn new(capacity: usize) -> MinHeap<T> {
        MinHeap (Vec::with_capacity(capacity))
    }

    fn insert(&mut self, elem: T) {
        let mut idx = self.0.len();
        self.0.push(elem);
        while idx > 0 {
            let predecessor_idx = if idx % 2 == 0 {(idx / 2) - 1} else {(idx / 2)};
            if self.0[predecessor_idx] > self.0[idx] {
                self.0.swap(predecessor_idx, idx);
                idx = predecessor_idx;
            } else { break; }
        };
    }

    fn remove(&mut self, index: usize) {
        let length = self.0.len();
        self.0.swap(index, length - 1);
        self.0.remove(length - 1);

        let (child1_idx, child2_idx) = (index * 2 + 1, index * 2 + 2);
        let child1_val = self.0[child1_idx];
        let child2_val = self.0[child2_idx];
        while self.0[index] > child1_val || self.0[index] > child2_val {
            if self.0[index] > child1_val && self.0[index] > child2_val {
                // Greater than both, therefore should go down and be swapped with the smallest of the two
                if child1_val > child2_val {
                    self.0.swap(index, child2_idx);
                } else {
                    self.0.swap(index, child1_idx);
                }
            } else if self.0[index] > child1_val && self.0[index] < child2_val {
                // Greater than v1 but not v2, change it with v1
                self.0.swap(index, child1_idx);
            } else if self.0[index] < child1_val && self.0[index] > child2_val {
                // Greater than v2 but not v1, change it with v2
                self.0.swap(index, child2_idx);
            }
        };
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binaryheap() {
        let mut h : MinHeap<u32> = MinHeap::new(20);
        h.insert(1);
        assert_eq!(&h.0, &vec![1]);
        h.insert(3);
        assert_eq!(&h.0, &vec![1, 3]);
        h.insert(4);
        assert_eq!(&h.0, &vec![1,3,4]);
        h.insert(6);
        assert_eq!(&h.0, &vec![1,3,4,6]);
        h.insert(0);
        assert_eq!(&h.0, &vec![0,1,4,6,3]);
        h.remove(0);
        assert_eq!(&h.0, &vec![1,3,4,6]);
    }
}