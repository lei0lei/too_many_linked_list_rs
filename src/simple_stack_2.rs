// 接下来为链表添加以下功能
// - 泛型支持
// - peek
// - 支持迭代器

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

// 实现时需要在impl块上添加泛型参数<T>,List<T>只是类型名
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|boxed_node| {
            // 这里解引用boxed_node，获得Node<T>所有权，可以返回整个Node，比如处理下面的情况
            // 1. Node字段比较多
            // 2. 把节点转移到另一个链表，缓存等地方
            // 3. 减少move和clone
            // 4. 实现链表底层操作如链表分割，拼接，批量处理等需要直接操作节点结构
            // let node = *boxed_node;
            self.head = boxed_node.next;
            boxed_node.elem
        })
    }

    // 整体 move Node<T> 的 pop
    pub fn pop_node(&mut self) -> Option<Node<T>> {
        self.head.take().map(|boxed_node| {
            let mut node = *boxed_node;
            self.head = node.next.take(); // 关键：断开链表，维护 head
            node
        })
        // 如果写成 self.head.take().map(|boxed_node| boxed_node) 会报类型不匹配错误
    }

    pub fn peek(&self) -> Option<&T> {
        // as_ref() 将 Option<Box<Node<T>>> 转换为 Option<&Box<Node<T>>>
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // as_mut() 将 Option<Box<Node<T>>> 转换为 Option<&mut Box<Node<T>>>
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }

}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);
// 实现值迭代器，消耗整个链表每次迭代返回一个元素的所有权
// for x in list.into_iter()
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// 只读借用迭代器，每次返回链表元素的不可变借用而不消耗链表本身
// for x in list.iter()
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iter<'a, T> {
    fn new(list: &'a List<T>) -> Self {
        Iter {
            next: list.head.as_deref(), // as_deref() 将 Option<Box<Node<T>>> 转换为 Option<&Node<T>>
        }
    }
}



// 可变借用迭代器，返回元素的可变引用，可以原地修改元素
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}
impl<'a, T> IterMut<'a, T> {
    fn new(list: &'a mut List<T>) -> Self {
        IterMut {
            next: list.head.as_deref_mut(), // as_deref_mut() 将 Option<Box<Node<T>>> 转换为 Option<&mut Node<T>>
        }
    }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn basics() {
       let mut list = List::new();

       // 检查空列表行为
       assert_eq!(list.pop(), None);

       // 填充列表
       list.push(1);
       list.push(2);
       list.push(3);

       // 检查正常弹出
       assert_eq!(list.pop(), Some(3));
       assert_eq!(list.pop(), Some(2));

       // 推入更多元素以确保没有问题
       list.push(4);
       list.push(5);

       // 检查剩余元素
       assert_eq!(list.pop(), Some(5));
       assert_eq!(list.pop(), Some(4));
       assert_eq!(list.pop(), Some(1));
       assert_eq!(list.pop(), None);
   }


    #[derive(Debug, PartialEq)]
    struct Person {
        name: String,
        age: u8,
    }


    #[test]
    fn complex_type() {
        let mut list = List::new();

        list.push(Person { name: "Alice".to_string(), age: 30 });
        list.push(Person { name: "Bob".to_string(), age: 25 });
        list.push(Person { name: "Carol".to_string(), age: 40 });

        assert_eq!(
            list.pop(),
            Some(Person { name: "Carol".to_string(), age: 40 })
        );
        assert_eq!(
            list.pop(),
            Some(Person { name: "Bob".to_string(), age: 25 })
        );
        assert_eq!(
            list.pop(),
            Some(Person { name: "Alice".to_string(), age: 30 })
        );
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn move_whole_node() {
        let mut list = List::new();
        list.push(Person { name: "Dave".to_string(), age: 50 });
        list.push(Person { name: "Eve".to_string(), age: 60 });

        let node = list.pop_node().unwrap();
        assert_eq!(node.elem, Person { name: "Eve".to_string(), age: 60 });
        assert_eq!(list.pop_node().unwrap().elem, Person { name: "Dave".to_string(), age: 50 });
        assert!(list.pop_node().is_none());
    }


    #[test]
    fn peek_and_peek_mut() {
        let mut list = List::new();
        assert!(list.peek().is_none());
        assert!(list.peek_mut().is_none());

        list.push(10);
        list.push(20);
        list.push(30);

        assert_eq!(list.peek(), Some(&30));
        assert_eq!(list.peek_mut(), Some(&mut 30));

        if let Some(top) = list.peek_mut() {
            *top = 100;
        }

        assert_eq!(list.peek(), Some(&100));
        assert_eq!(list.pop(), Some(100));
        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.pop(), None);
    }

}
