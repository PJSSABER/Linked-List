use std::mem;

type Link<T> = Option<Box<Node<T>>>;

struct List<T> {
    head: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            // next: self.head,     if use this, will occur an error!!  cannot move out of `self.head` which is behind a mutable reference, self.head will be dingling!!
            next: self.head.take(), // this equals to mem::replace(&mut self.head, None), Takes the value out of the option, leaving a None in its place.
        });

        self.head = Some(new_node); 
    }

    pub fn pop(&mut self) -> Option<T> {

        if let Some(tmp_node) = self.head.take() {
            self.head = tmp_node.next;
            return Some(tmp_node.elem);
        }
        /*  using map: Maps an Option<T> to Option<U> by applying a function to a contained value (if Some) or returns None (if None).
            self.head.take().map(|node| {
                self.head = node.next;
                node.elem
            });
        */
        return None;
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_node = self.head.take();  // Link<T>
        while let Some(mut node) = cur_node { // Box<T> 类型是一个智能指针，因为它实现了 Deref trait，它允许 Box<T> 值被当作引用对待。当 Box<T> 值离开作用域时，由于 Box<T> 类型 Drop trait 的实现，box 所指向的堆数据也会被清除。
            cur_node = node.next.take();
            // node goes out of scope and calls drop automatically
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
}
