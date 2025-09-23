pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

// 数据布局使用上面的结构有以下几个好处
// 1. List 结构体只包含一个指向堆上数据的指针，因此 List 的大小是固定的，适合在栈上分配
// 2. Link 使用 Option 包裹 Box<Node>，这样可以很方便地表示链表的结束（None）和节点的存在（Some(Box<Node>)）


impl List{
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
            // take() 方法会将 Option 的值取出，并将原来的 Option 置为 None
            // next: mem::replace(&mut self.head, None), // 另一种实现方式
        });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // 取出节点元素，把self.head 置为下一个节点
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }

        // 可以直接采用pop函数
        // while self.pop().is_some() {}
    }
}

// 测试代码
#[cfg(test)]
mod tests {
  use super::List;
    // 基本功能测试
    #[test]
    fn basics() {
        let mut list = List::new();

        // 检查空列表的弹出操作
        assert_eq!(list.pop(), None);

        // 填充列表
        list.push(1);
        list.push(2);
        list.push(3);

        // 检查正常的弹出操作
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // 推入更多元素
        list.push(4);
        list.push(5);

        // 检查剩余元素的弹出操作
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));

        // 确认列表现在为空
        assert_eq!(list.pop(), None);
    }
}