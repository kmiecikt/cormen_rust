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
        let new_node = Box::new(Node { value: value, next: self.head.take() });
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|mut head_node| {
            self.head = head_node.next.take();
            head_node.value
        })
    }
    
    pub fn peek(self: &List<T>) -> Option<&T> {
        self.head.as_ref().map(|head_node| &head_node.value)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_push_peek() {
        let mut list = List::new();
        assert_eq!(None, list.peek()); 

        list.push(1);
        assert_eq!(1, *list.peek().unwrap());

        list.push(2);
        assert_eq!(2, *list.peek().unwrap());
    }
    
    #[test]
    fn test_push_pop() {
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
}