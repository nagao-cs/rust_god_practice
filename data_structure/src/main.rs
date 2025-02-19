mod data_structures;

use data_structures::stack::Stack;
use data_structures::queue::Queue;

fn main() {
    let mut stack_a = Stack::new(2);
    stack_a.push(1);
    stack_a.push(2);
    stack_a.push(3);
    println!("{:?}", stack_a.pop());
    println!("{:?}", stack_a.pop());
    println!("{:?}", stack_a.pop());

    let mut queue: Queue = Queue::new();

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(5);
    println!("{}", queue.dequeue());
    println!("{}", queue.dequeue());
    println!("{}", queue.dequeue());
    println!("{}", queue.dequeue());
}