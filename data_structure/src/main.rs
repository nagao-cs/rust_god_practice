mod data_structures;

use data_structures::stack::Stack;
use data_structures::queue::Queue;
use data_structures::linkedlist::LinkedList;
use data_structures::ring_buffer::RingBuffer;

#[cfg(test)]
mod stack_tests {
    use super::*;

    #[test]
    fn stack_test() {
        let mut test_stack: Stack<i32> = Stack::new(3);
        test_stack.push(1);
        test_stack.push(2);
        test_stack.push(3);
        assert_eq!(test_stack.pop().unwrap(), 3);
        assert_eq!(test_stack.pop().unwrap(), 2);
        assert_eq!(test_stack.pop().unwrap(), 1);
    }

    #[test]
    #[should_panic]
    fn stack_overflow_test() {
        let mut test_stack: Stack<i32> = Stack::new(2);
        test_stack.push(1);
        test_stack.push(2);
        test_stack.push(3);
    }

    #[test]
    #[should_panic]
    fn stack_underflow_test() {
        let mut test_stack: Stack<i32> = Stack::new(1);
        test_stack.pop().unwrap();
    }
}

#[cfg(test)]
mod queue_tests {
    use super::*;

    #[test]
    fn queue_test() {
        let mut test_queue: Queue<i32> = Queue::new(3);
        test_queue.enqueue(1);
        test_queue.enqueue(2);
        test_queue.enqueue(3);
        assert_eq!(test_queue.dequeue().unwrap(), 1);
        assert_eq!(test_queue.dequeue().unwrap(), 2);
        assert_eq!(test_queue.dequeue().unwrap(), 3);
    }

    #[test]
    #[should_panic]
    fn queue_overflow_test() {
        let mut test_queue: Queue<i32> = Queue::new(2);
        test_queue.enqueue(1);
        test_queue.enqueue(2);
        test_queue.enqueue(3);
    }

    #[test]
    #[should_panic]
    fn queue_underflow_test() {
        let mut test_queue: Queue<i32> = Queue::new(1);
        test_queue.dequeue().unwrap();
    }
}

// #[cfg(test)]
// mod linkedlist_tests {
//     use super::*;

//     #[test]
//     fn linkedlist_test() {
//         let mut test_linkedlist: LinkedList<i32> = LinkedList::new();
//         test_linkedlist.push(1);
//         test_linkedlist.push(2);
//         test_linkedlist.push(3);
//         assert_eq!(test_linkedlist.pop().unwrap(), 3);
//         assert_eq!(test_linkedlist.pop().unwrap(), 2);
//         assert_eq!(test_linkedlist.pop().unwrap(), 1);
//     }

//     #[test]
//     #[should_panic]
//     fn linkedlist_underflow_test() {
//         let mut test_linkedlist: LinkedList<i32> = LinkedList::new();
//         test_linkedlist.pop().unwrap();
//     }
// }

#[cfg(test)]
mod ring_buffer_tests {
    use super::*;

    #[test]
    fn ring_buffer_test() {
        let mut test_ring_buffer: RingBuffer = RingBuffer::new(3);
        test_ring_buffer.enqueue(1).unwrap();
        test_ring_buffer.enqueue(2).unwrap();
        test_ring_buffer.enqueue(3).unwrap();
        assert_eq!(test_ring_buffer.dequeue().unwrap(), 1);
        assert_eq!(test_ring_buffer.dequeue().unwrap(), 2);
        assert_eq!(test_ring_buffer.dequeue().unwrap(), 3);
    }

    #[test]
    #[should_panic]
    fn ring_buffer_overflow_test() {
        let mut test_ring_buffer: RingBuffer = RingBuffer::new(2);
        test_ring_buffer.enqueue(1).unwrap();
        test_ring_buffer.enqueue(2).unwrap();
        test_ring_buffer.enqueue(3).unwrap();
    }

    #[test]
    #[should_panic]
    fn ring_buffer_underflow_test() {
        let mut test_ring_buffer: RingBuffer = RingBuffer::new(3);
        test_ring_buffer.enqueue(1).unwrap();
        test_ring_buffer.enqueue(2).unwrap();
        test_ring_buffer.enqueue(3).unwrap();
        test_ring_buffer.dequeue().unwrap();
        test_ring_buffer.dequeue().unwrap();
        test_ring_buffer.dequeue().unwrap();
        test_ring_buffer.dequeue().unwrap();
    }
}