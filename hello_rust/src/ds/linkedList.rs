use std::rc::Rc;

// single linkedlist
// https://course.rs/too-many-lists/persistent-stack/drop-arc.html
pub struct List<T> {
    head: Link<T>,
}

// alias
type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{ head: None } 
    }

    pub fn pretend(&self, val: T) -> List<T> {
        List {head: Some(Rc::new(Node {
            val: val,
            next: self.head.clone()
        }))}
    }

    pub fn tail(&self) -> List<T> {
        List {head: self.head.as_ref().and_then(|node| node.next.clone())}
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {next: self.head.as_deref()}
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

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) ->Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.val
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    
    #[test]
    fn basic() {
        let list = List::new();
        assert_eq!(None, list.head());

        let list = list.pretend(1).pretend(2).pretend(3);
        assert_eq!(Some(&3), list.head());

        let list = list.tail();
        assert_eq!(Some(&2), list.head());

        let list = list.tail();
        assert_eq!(Some(&1), list.head());

        let list = list.tail();
        assert_eq!(None, list.head());
    }

    #[test]
    fn iter() {
        let list = List::new().pretend(1).pretend(2).pretend(3);

        let mut iter = list.iter();
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
        assert_eq!(None, iter.next());
    }
}
