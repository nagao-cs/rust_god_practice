#[derive(Debug)]

struct Stack {
    top: usize,
    size: usize, 
    stack: Vec<Option<i32>>,
}

impl Stack {
    fn push(&mut self, data: i32) {
        if self.top < self.size {
            self.stack.push(Some(data));
            self.top += 1;
        }
        else {
            println!("Stack is full");
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if self.top > 0 {
            self.top -= 1;
            return self.stack[self.top];
        }
        else {
            println!("Stack is empty");
            return None;
        }
    }

    fn new(size: usize) -> Stack {
        Stack {
            top: 0,
            size: size,
            stack: Vec::new(),
        }
    }
}

fn main() {
    let mut stack_a = Stack::new(2);
    stack_a.push(1);
    stack_a.push(2);
    stack_a.push(3);
    
    match stack_a.pop() {
        Some(data) => println!("{:?}", data),
        None => (),
    }
    match stack_a.pop() {
        Some(data) => println!("{:?}", data),
        None => (),
    }
    match stack_a.pop() {
        Some(data) => println!("{:?}", data),
        None => (),
    }
}