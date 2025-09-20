/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/



use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node { val: t, next: None }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            length: 0,
            start: None,
            end: None,
        }
    }
}

impl<T: PartialOrd + Clone> LinkedList<T> {
    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        if index < 0 || index >= self.length as i32 {
            return None;
        }
        unsafe {
            let mut cur = self.start;
            for _ in 0..index {
                cur = (*cur.unwrap().as_ptr()).next;
            }
            Some(&(*cur.unwrap().as_ptr()).val)
        }
    }

    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        let mut merged = LinkedList::new();
        let mut cur_a = list_a.start;
        let mut cur_b = list_b.start;

        while cur_a.is_some() || cur_b.is_some() {
            match (cur_a, cur_b) {
                (None, Some(b)) => unsafe {
                    merged.add((*b.as_ptr()).val.clone());
                    cur_b = (*b.as_ptr()).next;
                },
                (Some(a), None) => unsafe {
                    merged.add((*a.as_ptr()).val.clone());
                    cur_a = (*a.as_ptr()).next;
                },
                (Some(a), Some(b)) => unsafe {
                    let va = &(*a.as_ptr()).val;
                    let vb = &(*b.as_ptr()).val;
                    if va <= vb {
                        merged.add(va.clone());
                        cur_a = (*a.as_ptr()).next;
                    } else {
                        merged.add(vb.clone());
                        cur_b = (*b.as_ptr()).next;
                    }
                },
                _ => break,
            }
        }
        merged
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &v in &vec_a {
            list_a.add(v);
        }
        for &v in &vec_b {
            list_b.add(v);
        }
        let list_c = LinkedList::merge(list_a, list_b);
        for (i, &v) in target_vec.iter().enumerate() {
            assert_eq!(Some(&v), list_c.get(i as i32));
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &v in &vec_a {
            list_a.add(v);
        }
        for &v in &vec_b {
            list_b.add(v);
        }
        let list_c = LinkedList::merge(list_a, list_b);
        for (i, &v) in target_vec.iter().enumerate() {
            assert_eq!(Some(&v), list_c.get(i as i32));
        }
    }
}