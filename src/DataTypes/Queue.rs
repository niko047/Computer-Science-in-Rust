
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



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn queue() {
        let mut q : Queue<u32> = Queue::new(10);
        q.push(32);
        assert_eq!(1, q.top);
        q.push(21);
        assert_eq!(q.pop(), 32);
        assert_eq!(q.pop(), 21);
    }

}