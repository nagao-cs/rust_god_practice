pub struct Queue {
    arr: [i32; 10],
    front: usize,
    rear: usize,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            arr: [0; 10],
            front: 0,
            rear: 0,
        }
    }

    pub fn enqueue(&mut self, d: i32) {
        if self.rear > self.arr.len() -1 {
            println!("Queue overflow!");
        }
        self.arr[self.rear] = d;
        self.rear += 1;
    }

    pub fn dequeue(&mut self) -> i32 {
        if self.front == self.rear {
            println!("Queue underflow!");
            return -1;
        }
        let x: i32 = self.arr[self.front];
        self.front += 1;
        x
    }
}