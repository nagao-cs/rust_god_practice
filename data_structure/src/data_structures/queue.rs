pub struct Queue<T: Clone> {
    queue: Vec<Option<T>>,
    front: usize,
    rear: usize,
}

impl<T: Clone> Queue<T> {
    pub fn new(size: usize) -> Queue<T> {
        Queue {
            queue: vec![None; size],
            front: 0,
            rear: 0,
        }
    }

    pub fn enqueue(&mut self, data: T) {
        if self.rear > self.queue.len() -1 {
            println!("Queue overflow!");
        }
        self.queue[self.rear] = Some(data);
        self.rear += 1;
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if self.front == self.rear {
            return Err("Queue underflow!");
        }
        let x = self.queue[self.front].take().ok_or("Queue underflow!");
        self.front += 1;
        return x;
    }
}