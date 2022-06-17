pub struct List<T> {
    head: Link<T>
}

struct Node<T> {
    pub value: T,
    pub next: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }
    
    pub fn push(&mut self, value: T) {
        let new_node = Box::new(Node { 
            value: value, 
            next: self.head.take() 
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }
    
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }
    
    pub fn reverse(&mut self) {
        let mut current = self.head.take();
        let mut previous = None;
        
        while let Some(mut current_node) = current {
            current = current_node.next.take();
            current_node.next = previous;
            previous = Some(current_node);
        }
        
        self.head = previous;
    }
    
    pub fn iter<'a>(&'a self) -> ListIterator<'a, T> {
        ListIterator { current: &self.head }
    }
    
    pub fn into_iter(self) -> ListIntoIterator<T> {
        ListIntoIterator { list: self }
    }
    
    pub fn iter_mut(&mut self) -> ListMutIterator<'_, T> {
        ListMutIterator { current: self.head.as_deref_mut() }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {
        }
    }
}

pub struct ListIterator<'a, T> {
    current: &'a Link<T>
}

impl<'a, T> Iterator for ListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.as_ref().map(|current_node| {
            self.current = &current_node.next;
            &current_node.as_ref().value
        })
    }
}

pub struct ListIntoIterator<T> {
    list: List<T>
}

impl<T> Iterator for ListIntoIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

pub struct ListMutIterator<'a, T> {
    current: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for ListMutIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn push_peek() {
        let mut list = List::new();
        assert_eq!(None, list.peek()); 

        list.push(1);
        assert_eq!(1, *list.peek().unwrap());

        list.push(2);
        assert_eq!(2, *list.peek().unwrap());
    }
    
    #[test]
    fn push_pop() {
        let mut list = List::new();
        assert_eq!(None, list.pop());
        
        list.push(1);
        list.push(2);
        list.push(3);
        
        assert_eq!(Some(3), list.pop());
        assert_eq!(Some(2), list.pop());
        assert_eq!(Some(1), list.pop());
        assert_eq!(None, list.pop());
    }
    
    #[test]
    fn reverse() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);
        list.reverse();
        
        assert_eq!(Some(1), list.pop());
        assert_eq!(Some(2), list.pop());
        assert_eq!(Some(3), list.pop());
        assert_eq!(None, list.pop());
    }
   
    #[test]
    fn iter() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);
        
        let mut iterator = list.iter();
        assert_eq!(Some(&3), iterator.next());
        assert_eq!(Some(&2), iterator.next());
        assert_eq!(Some(&1), iterator.next());
        assert_eq!(None, iterator.next());
    }
    
    #[test]
    fn into_iter() {
        let mut list = List::new();
        
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iterator = list.into_iter();
        assert_eq!(Some(3), iterator.next());
        assert_eq!(Some(2), iterator.next());
        assert_eq!(Some(1), iterator.next());
        assert_eq!(None, iterator.next());
        assert_eq!(None, iterator.next());
    }
    
    #[test]
    fn iter_mut() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut iterator = list.iter_mut();

        assert_eq!(Some(&mut 3), iterator.next());
        assert_eq!(Some(&mut 2), iterator.next());
        assert_eq!(Some(&mut 1), iterator.next());
        assert_eq!(None, iterator.next());
    }
}