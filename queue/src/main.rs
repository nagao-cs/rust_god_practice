struct Queue<T> {
    arr: Vec<T>,
    front: usize,
    rear: usize,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            arr: Vec::new(),
            front: 0,
            rear: 0,
        }
    }

    fn enqueue(&mut self, d: T) {
        self.arr.push(d);
        self.rear += 1;
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.arr.is_empty() {
            println!("Queue underflow!");
            None
        } else {
            Some(self.arr.remove(0))
        }
    }
} 

fn main() {
    let mut queue = Queue::new();

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(5);
    println!("{:?}", queue.dequeue());
    println!("{:?}", queue.dequeue());
    println!("{:?}", queue.dequeue());
    println!("{:?}", queue.dequeue());
}
