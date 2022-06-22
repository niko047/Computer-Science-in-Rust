
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
}