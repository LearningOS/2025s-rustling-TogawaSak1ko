/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

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

impl<T: std::cmp::PartialOrd+Clone> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::cmp::PartialOrd + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

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
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        //TODO
        let mut merged_list = LinkedList::new();

        let mut index_a = 0;
        let mut index_b = 0;
        let mut node_a = list_a.get(index_a);
        let mut node_b = list_b.get(index_b);

        loop {
            match (node_a, node_b) {
                (None, None) => {
                    break;
                }
                (None, Some(b)) => {
                    merged_list.add(b.clone());
                    index_b += 1;
                    node_b = list_b.get(index_b);
                }
                (Some(a), None) => {
                    merged_list.add(a.clone());
                    index_a += 1;
                    node_a = list_a.get(index_a);
                }
                (Some(a), Some(b)) => {
                    if a <= b {
                        merged_list.add(a.clone());
                        index_a += 1;
                        node_a = list_a.get(index_a);
                    } else {
                        merged_list.add(b.clone());
                        index_b += 1;
                        node_b = list_b.get(index_b);
                    }
                }
            }
        }
        merged_list
    }

    pub fn merge2(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();

        let mut index_a = 0;
        let mut index_b = 0;
        let mut node_a = list_a.get(index_a);
        let mut node_b = list_b.get(index_b);

        loop {
            if node_a.is_none() || node_b.is_none() {
                if let Some(a) = node_a {
                    merged_list.add(a.clone());
                    index_a += 1;
                    Self::appending_tail(&mut merged_list,&list_a,index_a);
                    break;
                }

                if let Some(b) = node_b {
                    merged_list.add(b.clone());
                    index_b += 1;
                    Self::appending_tail(&mut merged_list,&list_b,index_b);
                    break;
                }

                let a_val = node_a.unwrap();
                let b_val = node_b.unwrap();

                if a_val <= b_val {
                    merged_list.add(a_val.clone());
                    index_a += 1;
                    node_a = list_a.get(index_a);
                } else {
                    merged_list.add(b_val.clone());
                    index_b += 1;
                    node_b = list_b.get(index_b);
                }
            }
        }
        merged_list
    }

    fn appending_tail(res:&mut LinkedList<T>,list:&LinkedList<T>,mut index:i32) {
        while let Some(t) = list.get(index) {
            res.add(t.clone());
            index += 1;
        }
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

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
