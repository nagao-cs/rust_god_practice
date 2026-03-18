pub struct RingBuffer {
    buffer: Vec<Option<i32>>,
    rear: usize,
    front: usize,
    size: usize,
}

impl RingBuffer {
    pub fn new(size: usize) -> RingBuffer {
        RingBuffer {
            buffer: vec![None; size],
            rear: 0,
            front: 0,
            size: size,
        }
    }

    pub fn enqueue(&mut self, data: i32) -> Result<(), &str> {
        if self.rear - self.front == self.size {
            return Err("Ring buffer is full");
        }
        else {
            self.buffer[self.rear % self.size] = Some(data);
            self.rear += 1;
            return Ok(());
        }
    }

    pub fn dequeue(&mut self) -> Option<i32> {
        if self.rear == self.front {
            println!("Ring buffer is empty");
            return None;
        }
        else {
            let x: Option<i32> = self.buffer[self.front % self.size];
            self.front += 1;
            return x;
        }
    }
}