use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Node<T> {
        Self {
            element,
            next: None,
            prev: None,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, element: T) {
        let mut node = Box::new(Node::new(element));

        node.next = self.head;
        let node = Some(Box::leak(node).into());

        match self.head {
            None => self.tail = node,
            Some(head) => unsafe { (*head.as_ptr()).prev = node },
        }

        self.head = node;
        self.len += 1;
    }

    pub fn push_back(&mut self, element: T) {
        let mut node = Box::new(Node::new(element));

        node.prev = self.tail;
        let node = Some(Box::leak(node).into());

        match self.tail {
            None => self.head = node,
            Some(tail) => unsafe { (*tail.as_ptr()).next = node },
        }

        self.tail = node;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node| {
            let node = unsafe { Box::from_raw(node.as_ptr()) };

            self.head = node.next;
            match self.head {
                None => self.tail = None,
                Some(head) => unsafe { (*head.as_ptr()).prev = None },
            }

            self.len -= 1;
            node.element
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|node| {
            let node = unsafe { Box::from_raw(node.as_ptr()) };

            self.tail = node.prev;
            match self.tail {
                None => self.head = None,
                Some(tail) => unsafe { (*tail.as_ptr()).next = None },
            }

            self.len -= 1;
            node.element
        })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
            marker: PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Iter<'a, T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| {
                let node = unsafe { &*node.as_ptr() };
                self.head = node.next;
                self.len -= 1;

                &node.element
            })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| {
                let node = unsafe { &*node.as_ptr() };
                self.tail = node.prev;
                self.len -= 1;

                &node.element
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_must_push_front() {
        let mut linked_list: LinkedList<u64> = LinkedList::new();

        assert_eq!(linked_list.len(), 0);

        linked_list.push_front(1);
        linked_list.push_front(2);
        linked_list.push_front(3);

        assert_eq!(linked_list.len(), 3);
    }

    #[test]
    fn linked_list_must_pop_front() {
        let mut linked_list: LinkedList<u64> = LinkedList::new();

        assert_eq!(linked_list.len(), 0);
        assert_eq!(linked_list.pop_front(), None);

        linked_list.push_front(1);
        linked_list.push_front(2);
        linked_list.push_front(3);

        assert_eq!(linked_list.len(), 3);

        assert_eq!(linked_list.pop_front(), Some(3));
        assert_eq!(linked_list.pop_front(), Some(2));
        assert_eq!(linked_list.pop_front(), Some(1));
        assert_eq!(linked_list.pop_front(), None);

        assert_eq!(linked_list.len(), 0);
    }

    #[test]
    fn linked_list_must_push_back() {
        let mut linked_list: LinkedList<u64> = LinkedList::new();

        assert_eq!(linked_list.len(), 0);

        linked_list.push_back(1);
        linked_list.push_back(2);
        linked_list.push_back(3);

        assert_eq!(linked_list.len(), 3);
    }

    #[test]
    fn linked_list_must_pop_back() {
        let mut linked_list: LinkedList<u64> = LinkedList::new();

        assert_eq!(linked_list.len(), 0);
        assert_eq!(linked_list.pop_back(), None);

        linked_list.push_back(1);
        linked_list.push_back(2);
        linked_list.push_back(3);

        assert_eq!(linked_list.len(), 3);

        assert_eq!(linked_list.pop_back(), Some(3));
        assert_eq!(linked_list.pop_back(), Some(2));
        assert_eq!(linked_list.pop_back(), Some(1));
        assert_eq!(linked_list.pop_back(), None);

        assert_eq!(linked_list.len(), 0);
    }

    #[test]
    fn linked_list_must_push_front_and_pop_back() {
        let mut linked_list: LinkedList<u64> = LinkedList::new();

        assert_eq!(linked_list.len(), 0);

        linked_list.push_front(1);
        linked_list.push_front(2);
        linked_list.push_front(3);

        assert_eq!(linked_list.len(), 3);

        assert_eq!(linked_list.pop_back(), Some(1));
        assert_eq!(linked_list.pop_back(), Some(2));
        assert_eq!(linked_list.pop_back(), Some(3));
        assert_eq!(linked_list.pop_back(), None);

        assert_eq!(linked_list.len(), 0);
    }

    #[test]
    fn linked_list_must_push_back_and_pop_front() {
        let mut linked_list: LinkedList<u64> = LinkedList::new();

        assert_eq!(linked_list.len(), 0);

        linked_list.push_back(1);
        linked_list.push_back(2);
        linked_list.push_back(3);

        assert_eq!(linked_list.len(), 3);

        assert_eq!(linked_list.pop_front(), Some(1));
        assert_eq!(linked_list.pop_front(), Some(2));
        assert_eq!(linked_list.pop_front(), Some(3));
        assert_eq!(linked_list.pop_front(), None);

        assert_eq!(linked_list.len(), 0);
    }

    #[test]
    fn linked_list_iter() {
        let mut linked_list: LinkedList<u64> = LinkedList::new();

        linked_list.push_back(1);
        linked_list.push_back(2);
        linked_list.push_back(3);

        let mut iter = linked_list.iter();

        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn linked_list_iter_back() {
        let mut linked_list: LinkedList<u64> = LinkedList::new();

        linked_list.push_front(1);
        linked_list.push_front(2);
        linked_list.push_front(3);

        let mut iter = linked_list.iter();

        assert_eq!(iter.next_back(), Some(&1));
        assert_eq!(iter.next_back(), Some(&2));
        assert_eq!(iter.next_back(), Some(&3));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn linked_list_iter_forward_backward_intertwined() {
        let mut linked_list: LinkedList<u64> = LinkedList::new();

        linked_list.push_front(1);
        linked_list.push_front(2);
        linked_list.push_front(3);

        let mut iter = linked_list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next_back(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
