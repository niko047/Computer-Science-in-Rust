use std::cmp::{max, min};

fn main() {
}


/*
Implementation in rust of the following concepts:
1. Linked List - Single and Doubly
2. Stack
3. Queues
4. Binary Search Trees or general Binary Tree
5. Heaps
6. Basic Graph Traversal and Shortest Path
7. Hashing : (HashSets and HashMaps)
*/


/* Linked List, definition:
In computer science, a linked list is a linear collection of data elements whose order is not
given by their physical placement in memory.
Instead, each element points to the next.
It is a data structure consisting of a collection of nodes which together represent a sequence.
In its most basic form, each node contains: data, and a reference (in other words, a link) to the next node in the sequence
*/


struct LinkedListItem<'a, T> {
    value: T,
    pointer: Option<&'a LinkedListItem<'a, T>>
}

impl<'a, T> LinkedListItem<'a, T> {
    pub fn new(value: T) -> Self {
        LinkedListItem {
            value, // Note that this is equivalent to value: value
            pointer: None,
        }
    }

    pub fn add_link(&mut self, pointer: &'a LinkedListItem<T>) {
        match self.pointer.as_ref() {
            Some(_) => panic!("Link already exists"),
            None => self.pointer = Some(pointer),
        }
    }
}


/* Stack, definition:
In computer science, a stack is an abstract data type that serves as a collection of elements,
with two main principal operations:

Push, which adds an element to the collection, and
Pop, which removes the most recently added element that was not yet removed.
The order in which elements come off a stack gives rise to its alternative name, LIFO (last in, first out)
*/


struct Stack<T> {
    maxsize: usize,
    top: usize,
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new(maxsize: usize) -> Self {
        Stack {
            maxsize,
            top: 0 as usize,
            items: Vec::<T>::with_capacity(maxsize),
        }
    }

    fn push(&mut self, element: T) {
        if self.maxsize == self.items.len() {
            panic!("Tried to insert item in array when full");
        }
        self.items.insert(0, element);
        self.top += 1;
    }

    fn pop(&mut self) -> T {
        if self.items.len() == 0 {
            panic!("Tried to remove item from empty stack");
        };
        self.top -= 1;
        self.items.remove(0)
    }
}


/* Queues, definition:
In computer science, a queue is a collection of entities that are maintained in a sequence
and can be modified by the addition of entities at one end of the sequence and the removal of entities
from the other end of the sequence.
The operations of a queue make it a first-in-first-out (FIFO) data structure.
*/

struct Queue<T>{
    maxsize: usize,
    top: usize,
    items: Vec<T>,
}


impl<T> Queue<T> {
    pub fn new(maxsize: usize) -> Self {
        Queue {
            maxsize,
            top: 0,
            items: Vec::<T>::with_capacity(maxsize),
        }
    }

    fn push(&mut self, item: T) {
        if self.items.len() == self.maxsize {
            panic!("Queue is full, cannot insert further");
        };
        self.items.insert(0, item);
        self.top += 1;
    }

    fn pop(&mut self) -> T {
        if self.top == 0 {
            panic!("Queue is empty, cannot pop further");
        };
        let output = self.items.remove(self.top - 1);
        self.top -= 1;
        output
    }
}

/* Binary tree, definition:
In computer science, a binary tree is a tree data structure in which each node has at most two children,
which are referred to as the left child and the right child
 */


enum Side {
    Right,
    Left,
}

#[derive(Debug)]
struct TreeNode<T> {
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
    value: T,
}

impl<T> TreeNode<T> {
    fn new(value:T) -> Self {
        TreeNode {
            left: None,
            right: None,
            value,
        }
    }

    fn add(&mut self, tree: Box<TreeNode<T>>, side: Side) {
        match side {
            Side::Right => {self.right = Some(tree);},
            Side::Left=> {self.left = Some(tree);},
        }
    }
}


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
    fn linkedlist() {
        let mut i1 = LinkedListItem::new(5);
        let mut i2 = LinkedListItem::new(12);
        let mut i3 = LinkedListItem::new(27);
        i1.add_link(&i2);
        i2.add_link(&i3);
        assert_eq!(i2.value, 12);
    }
    #[test]
    fn stack() {
        let mut s : Stack<u32> = Stack::new(10);
        s.push(12);
        s.push(54);
        assert_eq!(s.pop(), 54);
        assert_eq!(s.top, s.items.len());
        assert_eq!(s.pop(), 12);
    }

    #[test]
    fn queue() {
        let mut q : Queue<u32> = Queue::new(10);
        q.push(32);
        assert_eq!(1, q.top);
        q.push(21);
        assert_eq!(q.pop(), 32);
        assert_eq!(q.pop(), 21);
    }

    #[test]
    fn binarytree() {
        let mut t1 = TreeNode::new(0);
        let mut t2 = TreeNode::new(10);
        let mut t3 = TreeNode::new(7);
        t1.add(Box::new(t2), Side::Left);
        t1.add(Box::new(t3), Side::Right);
        println!("{:?}", &t1);
        assert_eq!(t1.left.unwrap().value, 10);
        assert_eq!(t1.right.unwrap().value, 7);
    }

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