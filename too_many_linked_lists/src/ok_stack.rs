use std::fmt::Debug;

// List is a struct with a single field, its size is the same as that field.
// zero-cost abstractions!
struct Stack<T> {
    head: Link<T>,
}

// Link is in null-pointer-optimized form, which eliminates the space needed for the tag.
// All elements are uniformly allocated. All Node objects are allocated on the heap,
// and All List objects are not boxed.
type Link<T> = Option<Box<Node<T>>>;

// To have all Node objects boxed is more efficient, because when we manipulate the List object,
// we only play around with Box<>, which is a pointer. If we don't use Box<>, each time we
// manipulate List, we need to box/unbox object, copying object from/to stack/heap is costly.
#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { head: None }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    pub fn push(&mut self, val: T) {
        // used Box::new here instead of later to avoid allocation in stack.
        let new_head = Box::new(Node {
            val,
            next: self.head.take(),
        });

        self.head = Some(new_head)
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }
}

impl<T> Drop for Stack<T> {
    // Because all the element in the LinkedListStack implements Drop,
    // we do not actually have to implement Drop for LinkedListStack,
    // however, the implicit implementation of Drop may be recursive,
    // therefore, we explicitly implement an iterative version.
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(mut node) = head {
            head = node.next.take();
            // drop(node); not required, node will be out of scopy after
            // each iteration, so drop will be called automatically.
        }
    }
}

// we can only implement Iterator once for a type, in order to have all three
// different iterators, into_iter, iter, and iter_mut, we use three auxiliary
// struct to achieve that.
impl<T> Stack<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            cur_node: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            cur_node: self.head.as_deref_mut(),
        }
    }
}

struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T> {
    cur_node: Option<&'a Node<T>>,
}

impl<'a, T: Debug> Iterator for Iter<'a, T> {
    type Item = &'a T;

    // fn next<'b>(&'b mut self) -> Option<&'a T>
    // no constrain on the lifetime of input and output, it means that we can call
    // next over and over unconditionally. This is fine for shared reference.
    fn next(&mut self) -> Option<Self::Item> {
        // & implemented Copy trait, therefore, Option<&> is also a Copy,
        // no need to use self.cur_node.take().map() here.
        self.cur_node.map(|node| {
            // self.cur_node = node.next.as_ref().map(|node| &**node);
            // self.cur_node = node.next.as_ref().map::<&Node<T>, _>(|node| node);
            self.cur_node = node.next.as_deref();
            &node.val
        })
    }
}

struct IterMut<'a, T> {
    cur_node: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // &mut does not implement Copy trait, therefore, we have to use take() here.
        self.cur_node.take().map(|node| {
            self.cur_node = node.next.as_deref_mut();
            &mut node.val
        })
    }
}

#[cfg(test)]
mod ok_stack_test {
    use super::Stack;

    #[test]
    fn basics() {
        let mut lls = Stack::new();
        assert_eq!(lls.pop(), None);
        lls.push(1);

        lls.push(2);
        lls.push(3);
        assert_eq!(lls.pop(), Some(3));
        assert_eq!(lls.pop(), Some(2));

        lls.push(4);
        assert_eq!(lls.pop(), Some(4));
        assert_eq!(lls.pop(), Some(1));
        assert_eq!(lls.pop(), None);
    }

    #[test]
    fn peek() {
        let mut lls = Stack::new();
        assert_eq!(lls.peek(), None);
        lls.push(1);
        assert_eq!(lls.peek(), Some(&1));
    }

    #[test]
    fn peek_mut() {
        let mut lls = Stack::new();
        assert_eq!(lls.peek_mut(), None);
        lls.push(1);
        if let Some(top) = lls.peek_mut() {
            *top = 99;
            assert_eq!(lls.peek(), Some(&99));
        }
    }

    #[test]
    fn into_iter() {
        let mut lls = Stack::new();
        lls.push(1);
        lls.push(2);
        lls.push(3);

        let mut ii = lls.into_iter();
        assert_eq!(ii.next(), Some(3));
        assert_eq!(ii.next(), Some(2));
        assert_eq!(ii.next(), Some(1));
        assert_eq!(ii.next(), None);
    }

    #[test]
    fn iter() {
        let mut lls = Stack::new();
        let mut i = lls.iter();
        assert_eq!(i.next(), None);
        lls.push(1);
        lls.push(2);

        i = lls.iter();
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), None);

        assert_eq!(lls.pop(), Some(2));
    }

    #[test]
    fn iter_mut() {
        let mut lls = Stack::new();
        let mut i = lls.iter_mut();
        assert_eq!(i.next(), None);
        lls.push(1);
        lls.push(2);

        i = lls.iter_mut();
        let head = i.next();
        assert_eq!(head, Some(&mut 2));
        head.map(|node| *node += 9);
        assert_eq!(i.next(), Some(&mut 1));
        assert_eq!(i.next(), None);

        assert_eq!(lls.pop(), Some(11));
    }
}
