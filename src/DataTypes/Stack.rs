
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stack() {
        let mut s : Stack<u32> = Stack::new(10);
        s.push(12);
        s.push(54);
        assert_eq!(s.pop(), 54);
        assert_eq!(s.top, s.items.len());
        assert_eq!(s.pop(), 12);
    }

}