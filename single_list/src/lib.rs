use std::mem;
use core::any::type_name;

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

    // 头插法
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
            Map takes self by value, which would move the Option out of the thing it's in
            self.head.take().map(|node| {
                self.head = node.next;   here node will do a unwrap automatically
                node.elem
            });
        */
        return None;
    }

    /*
    impl<T> Option<T> {
        pub fn as_ref(&self) -> Option<&T>;
    }

    using as_ref and as_mut leave it where it was
     */
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
            // println!("{}", type_name(node));
        } )
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
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


// IntoIter
pub struct IntoIter<T> (List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    } 
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.pop()   // self.0 changed
    }
}

// Iter     Option<Box<Node<T>>>
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter<'a>(&'a mut self) -> Iter<'a, T> {
        Iter {next: self.head.as_deref().map(|node| node)}
    }
}

// pub fn as_deref(&self) -> Option<&<T as Deref>::Target>
// type 被 option包裹时， T 无法自动deref, 此时显示调用 .as_deref()
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref().map(|node| node);
            &node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::<i32>::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::<i32>::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1); 
        list.push(2); 
        list.push(3);

        // println!("***************** {}", list.peek().unwrap());
        // println!("*****************");
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        list.peek_mut().map(|value| {
            *value = 42
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
    
}
