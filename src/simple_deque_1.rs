// 安全的双向连接队列
// 主要关注Refcell的使用，Refcell允许在编译时不可变的情况下，在运行时进行可变借用检查
// Refcell的核心是两个方法
// fn borrow(&self) -> Ref<'_, T>;
// fn borrow_mut(&self) -> RefMut<'_, T>;


use std::rc::Rc;
use std::cell::RefCell;


pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            if let Some(next) = old_head.borrow_mut().next.take() {
                next.borrow_mut().prev.take();
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<std::cell::Ref<T>> {
        self.head.as_ref().map(|node| {
            std::cell::Ref::map(node.borrow(), |n| &n.elem)
        })
    }

    pub fn peek_front_mut(&self) -> Option<std::cell::RefMut<T>> {
        self.head.as_ref().map(|node| {
            std::cell::RefMut::map(node.borrow_mut(), |n| &mut n.elem)
        })
    }

    pub fn push_back(&mut self, elem: T) {
        let new_node = Node::new(elem);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }   

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            if let Some(prev) = old_tail.borrow_mut().prev.take() {
                prev.borrow_mut().next.take();
                self.tail = Some(prev);
            } else {
                self.head.take();
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_back(&self) -> Option<std::cell::Ref<T>> {
        self.tail.as_ref().map(|node| {
            std::cell::Ref::map(node.borrow(), |n| &n.elem)
        })
    }

    pub fn peek_back_mut(&self) -> Option<std::cell::RefMut<T>> {
        self.tail.as_ref().map(|node| {
            std::cell::RefMut::map(node.borrow_mut(), |n| &mut n.elem)
        })
    }

}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(*list.peek_front().unwrap(), 3);
        assert_eq!(*list.peek_back().unwrap(), 1);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), None);

        list.push_back(4);
        list.push_back(5);
        list.push_back(6);
        assert_eq!(*list.peek_front().unwrap(), 4);
        assert_eq!(*list.peek_back().unwrap(), 6);
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_back(), Some(6));
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), None);

        list.push_front(7);
        list.push_back(8);
        list.push_front(9);
        assert_eq!(*list.peek_front().unwrap(), 9);
        assert_eq!(*list.peek_back().unwrap(), 8);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(9));
        assert_eq!(iter.next_back(), Some(8));
        assert_eq!(iter.next(), Some(7));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}