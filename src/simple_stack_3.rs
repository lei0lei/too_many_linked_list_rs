// 持久化列表实现，主要关注Rc的使用
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
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
        self.head.as_ref().map(|node| &node.elem)
    }
}
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(node) = cur_link {
            // 尝试减少引用计数，如果成功，说明没有其他引用，可以安全地继续拆解节点
            if let Ok(mut node) = Rc::try_unwrap(node) {
                cur_link = node.next.take();
            } else {
                break; // 还有其他引用，停止拆解
            }
        }
    }   
}

#[cfg(test)]
mod test{
   use super::*;
    #[test]
    fn test() {
        let list = List::new();
        let list = list.prepend(1);
        let list = list.prepend(2);
        let list = list.prepend(3);

        assert_eq!(list.head(), Some(&3));
        let tail = list.tail();
        assert_eq!(tail.head(), Some(&2));
        let tail2 = tail.tail();
        assert_eq!(tail2.head(), Some(&1));
        let tail3 = tail2.tail();
        assert_eq!(tail3.head(), None);
    }
   
    
}