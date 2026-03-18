#[derive(Debug)]

struct RingBuffer {
    buffer: Vec<Option<i32>>,
    rear: usize,
    front: usize,
    size: usize,
}

impl RingBuffer {
    fn new(size: usize) -> RingBuffer {
        RingBuffer {
            buffer: vec![None; size],
            rear: 0,
            front: 0,
            size: size,
        }
    }

    fn enqueue(&mut self, data: i32) {
        if self.rear - self.front == self.size {
            println!("Ring buffer is full");
        }
        else {
            self.buffer[self.rear % self.size] = Some(data);
            self.rear += 1;
        }
    }

    fn dequeue(&mut self) -> Option<i32> {
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

fn main() {
    let mut ring_buffer = RingBuffer::new(3);

    ring_buffer.enqueue(1);
    ring_buffer.enqueue(2);
    ring_buffer.enqueue(3);
    ring_buffer.enqueue(4);
    // dbg!(&ring_buffer.buffer);
    println!("{:?}", ring_buffer.dequeue());
    println!("{:?}", ring_buffer.dequeue());
    println!("{:?}", ring_buffer.dequeue());
    println!("{:?}", ring_buffer.dequeue());
}