pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut next = &self.head;

        while let Some(node) = next {
            len += 1;
            next = &node.next;
        }

        return len as usize;
    }

    pub fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            data: element,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut list: SimpleLinkedList<T> = SimpleLinkedList::new();
        let mut next = &self.head;

        while let Some(node) = next {
            list.push(node.data.clone());
            next = &node.next;
        }

        return list;
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(elements: &[T]) -> Self {
        let mut list: SimpleLinkedList<T> = SimpleLinkedList::new();

        for element in elements {
            list.push(element.clone());
        }

        return list;
    }
}

impl<T: Clone> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut list = Vec::<T>::new();
        let mut next = &self.rev().head;

        while let Some(node) = next {
            list.push(node.data.clone());
            next = &node.next;
        }

        return list
    }
}
