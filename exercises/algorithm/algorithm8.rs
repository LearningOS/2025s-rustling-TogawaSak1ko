/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
// I AM DONE

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        let (emplaced_queue,used_queue) = if self.q1.is_empty() {
            (&mut self.q1,&mut self.q2)
        } else {
            (&mut self.q2,&mut self.q1)
        };

        emplaced_queue.enqueue(elem);

        while let Ok(used_item) = used_queue.dequeue() {
            emplaced_queue.enqueue(used_item);
        }  
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        let mut q = if self.q1.is_empty() {
            &mut self.q2
        } else {
            &mut self.q1
        };

        match q.dequeue() {
            Ok(e) => {return Ok(e);},
            Err(_) => {return Err("Stack is empty");}
        }
		
    }
    pub fn is_empty(&self) -> bool {
		//TODO
        // true
        return self.q1.is_empty() && self.q2.is_empty();
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}