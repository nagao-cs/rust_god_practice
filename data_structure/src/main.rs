mod data_structures;

use data_structures::stack::Stack;
use data_structures::queue::Queue;
use data_structures::linkedlist::LinkedList;
use data_structures::ring_buffer::RingBuffer;

fn main() {
    println!("Stack");
    let mut stack_a = Stack::new(2);
    stack_a.push(1);
    stack_a.push(2);
    stack_a.push(3);
    println!("{:?}", stack_a.pop());
    println!("{:?}", stack_a.pop());
    println!("{:?}", stack_a.pop());

    println!("Queue");
    let mut queue: Queue = Queue::new();
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(5);
    println!("{}", queue.dequeue());
    println!("{}", queue.dequeue());
    println!("{}", queue.dequeue());
    println!("{}", queue.dequeue());

    println!("linkedlist");
    let mut list = LinkedList::new_list();
    list.insert_head(1);
    list.insert_head(2);
    list.insert_head(3);
    println!("{:?}", list);

    if let Some(cell) = list.search_cell(2) {
        cell.insert(9);
    }
    println!("{:?}", list);

    if let Some(cell) = list.search_cell(2) {
        cell.delete();
    }
    println!("{:?}", list);

    println!("RingBuffer");
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