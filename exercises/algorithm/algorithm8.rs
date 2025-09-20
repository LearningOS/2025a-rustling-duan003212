/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


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

pub struct myStack<T> {
    q1: Queue<T>,
    q2: Queue<T>,
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        myStack {
            q1: Queue::new(),
            q2: Queue::new(),
        }
    }

    pub fn push(&mut self, elem: T) {
       
        if !self.q1.is_empty() {
            self.q1.enqueue(elem);
        } else {
            self.q2.enqueue(elem);
        }
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        if self.is_empty() {
            return Err("Stack is empty");
        }
       
        let (src, dst) = if !self.q1.is_empty() {
            (&mut self.q1, &mut self.q2)
        } else {
            (&mut self.q2, &mut self.q1)
        };

        
        while src.size() > 1 {
            let x = src.dequeue().unwrap();
            dst.enqueue(x);
        }
       
        src.dequeue()
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
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