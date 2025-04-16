/*
	heap
	This question requires you to implement a binary heap function
*/
// I AM DONE

use std::cmp::Ord;
use std::default::Default;
#[derive(Debug)]
pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}
const HEAD_INDEX:usize = 1;
impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count += 1;
        self.pop_up(self.count);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		// 0
        let mut parent_index = idx;

        let l_child_index = self.left_child_idx(parent_index);
        if l_child_index > self.count {
            return 0;
        }
        
        let r_child_index = l_child_index + 1;
        if r_child_index > self.count || (self.comparator)(&self.items[l_child_index],&self.items[r_child_index]) {
            return l_child_index;
        }
        return r_child_index;
    }
    fn swim_down(&mut self,idx:usize) {
        let mut parent_index = idx;
        // let mut smallest_child_index = self.smallest_child_idx(parent_index);

        // while smallest_child_index > 0 && (self.comparator)(&self.items[smallest_child_index],&self.items[parent_index]) {
        //     self.items.swap(smallest_child_index,parent_index);
        //     parent_index = smallest_child_index;
        //     smallest_child_index = self.smallest_child_idx(parent_index);
        // }

        loop {
            let mut child_index = self.smallest_child_idx(parent_index);
            if (child_index == 0 || !(self.comparator)(&self.items[child_index],&self.items[parent_index])) {
                break;
            }
            self.items.swap(child_index,parent_index);
            parent_index = child_index;
        }
    }

    fn pop_up(&mut self,idx:usize) {
        // if idx == HEAD_INDEX {
        //     return ;
        // }

        let mut curr = idx;
        while curr > HEAD_INDEX {
            let parent_index = self.parent_idx(curr);
            if (self.comparator)(&self.items[curr],&self.items[parent_index]) {
                self.items.swap(curr,parent_index);
                curr = parent_index;
            } else {
                break;
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		// None
        if self.is_empty() {
            return None;
        }

        self.items.swap(HEAD_INDEX,self.count);
        let old_head = self.items.pop();
        self.count -= 1;
        if self.count > 0 {
            self.swim_down(HEAD_INDEX);
        }

        return old_head;
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
    #[test]
    fn test_max_heap2() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        heap.add(12);
        assert_eq!(heap.len(), 5);
        assert_eq!(heap.next(), Some(12));
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(2));
        // heap.add(1);
        // assert_eq!(heap.next(), Some(2));
        // println!("{:?}",heap.next());
        // println!("{:?}",heap.next());
        // println!("{:?}",heap.next());
        // println!("{:?}",heap.next());
    }
}