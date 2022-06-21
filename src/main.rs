use std::cmp::max;

fn main() {
    let mut t1 = TreeNode::new(0);
    let mut t2 = TreeNode::new(10);
    let mut t3 = TreeNode::new(7);
    println!("{:?}", &t1);
    t1.add(Box::new(t2), Side::Left);
    println!("{:?}", &t1);
    t1.add(Box::new(t3), Side::Right);
    println!("{:?}", &t1);
    let mut t4 = t1;
    let t4 = create_add(t4);
    println!("{:?}", &t4);
}

fn create_add(mut tree: TreeNode<u32>) -> TreeNode<u32>{
    let t = TreeNode {
        left:None,
        right:None,
        value:99
    };
    tree.right = Some(Box::new(t));
    tree
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
}