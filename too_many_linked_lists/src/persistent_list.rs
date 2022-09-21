// A persistent immutable singly-linekd list

use std::rc::Rc;

struct List<T> {
    head: Option<Rc<Node<T>>>,
}

struct Node<T> {
    val: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn prepend(&mut self, val: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                val,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            cur_node: self.head.as_deref(),
        }
    }
}

struct Iter<'a, T> {
    cur_node: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // following commented is what I wrote, which contains too many
        // unneccessary steps.
        // self.cur_node.take().map(|node| {
        //     self.cur_node = node.next.as_deref().map(|next| next);
        //     &node.val
        // })
        self.cur_node.map(|node| {
            self.cur_node = node.next.as_deref();
            &node.val
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod ps_test {
    use crate::persistent_list::List;

    #[test]
    fn basics() {
        let mut l = List::new();
        assert_eq!(l.head(), None);
        l = l.prepend(1).prepend(2);
        assert_eq!(l.head(), Some(&2));

        let mut ll = l.tail();
        assert_eq!(ll.head(), Some(&1));
        ll = ll.tail();
        assert_eq!(ll.head(), None);
    }

    #[test]
    fn iter() {
        let l = List::new().prepend(1).prepend(2).prepend(3);

        let mut i = l.iter();
        assert_eq!(i.next(), Some(&3));
        assert_eq!(i.next(), Some(&2));
        assert_eq!(i.next(), Some(&1));
        assert_eq!(i.next(), None);
    }
}
