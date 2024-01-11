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
            next: self.head.take(), // this equals to mem::replace(&mut self.head, None)
        });

        self.head = Some(new_node); 
    }

    pub fn pop(&mut self) -> Option<T> {

        if let Some(node) = &mut self.head {
            self.head = (*node).next.take();

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
}
