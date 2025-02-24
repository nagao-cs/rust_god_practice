mod data_structures;

use data_structures::stack::Stack;
use data_structures::queue::Queue;
use data_structures::linkedlist::LinkedList;
use data_structures::ring_buffer::RingBuffer;
use data_structures::tree::Tree;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tree() {
        let tree = Tree::new(10);
        assert_eq!(tree.get_label(), &10);
        assert!(tree.get_left().is_none());
        assert!(tree.get_right().is_none());
    }

    #[test]
    fn test_insert_left() {
        let mut tree = Tree::new(10);
        tree.insert(5);
        assert!(tree.get_left().is_some());
        assert_eq!(tree.get_left().unwrap().get_label(), &5);
    }

    #[test]
    fn test_insert_right() {
        let mut tree = Tree::new(10);
        tree.insert(15);
        assert!(tree.get_right().is_some());
        assert_eq!(tree.get_right().unwrap().get_label(), &15);
    }

    #[test]
    fn test_insert() {
        let mut tree = Tree::new(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(3);
        tree.insert(7);
        tree.insert(12);
        tree.insert(18);

        assert_eq!(tree.get_left().unwrap().get_label(), &5);
        assert_eq!(tree.get_left().unwrap().get_left().unwrap().get_label(), &3);
        assert_eq!(tree.get_left().unwrap().get_right().unwrap().get_label(), &7);

        assert_eq!(tree.get_right().unwrap().get_label(), &15);
        assert_eq!(tree.get_right().unwrap().get_left().unwrap().get_label(), &12);
        assert_eq!(tree.get_right().unwrap().get_right().unwrap().get_label(), &18);
    }

    #[test]
    fn test_print() {
        let mut tree = Tree::new(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(3);
        tree.insert(7);
        tree.insert(12);
        tree.insert(18);

        tree.print();
    }
}
