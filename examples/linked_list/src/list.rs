pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&mut self, elem: T) {
        let new = Box::new(Node {
            elem,
            next: self.head.take()  // same as `mem::replace(&mut self.head, None)`
        });

        self.head = Some(new);
    }

    fn pop(&mut self) -> Option<T> {
        // Map over an Option in Rust has the same effect as fmap over a Maybe in Haskell.
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}


#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);
        
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
