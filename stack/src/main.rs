#[derive(Debug)]

enum Stackable {
    Int(i32),
    Float(f32),
    
}

struct Stack {
    top: usize,
    size: usize, 
    stack: Vec<Option<Stackable>>,
}

impl Stack {
    fn push(&mut self, data: Stackable) {
        if self.is_full() {
            println!("Stack is full");
        }
        else {
            // self.stack[self.top] = Some(data);
            self.stack.push(Some(data));
            self.top += 1;
        }
    }

    fn pop(&mut self) -> Option<Stackable> {
        if self.is_empty() {
            println!("Stack is empty");
            return None;
        }
        else {
            self.top -= 1;
            return self.stack[self.top].take();
        }
    }

    fn new(size: usize) -> Stack {
        Stack {
            top: 0,
            size: size,
            stack: Vec::with_capacity(size),
        }
    }

    fn is_full(&self) -> bool {
        if self.top == self.size {
            return true;
        }
        else {
            return false;
        }
    }

    fn is_empty(&self) -> bool {
        if self.top == 0 {
            return true;
        }
        else {
            return false;
        }
    }
}

fn main() {
    let mut stack_a = Stack::new(2);
    stack_a.push(Stackable::Int(1));
    stack_a.push(Stackable::Float(2.0));
    stack_a.push(Stackable::Int(3));
    match stack_a.pop() {
        Some(Stackable::Int(data)) => println!("{:?}", data),
        Some(Stackable::Float(data)) => println!("{:?}", data),
        None => (),
    }
    match stack_a.pop() {
        Some(Stackable::Int(data)) => println!("{:?}", data),
        Some(Stackable::Float(data)) => println!("{:?}", data),
        None => (),
    }match stack_a.pop() {
        Some(Stackable::Int(data)) => println!("{:?}", data),
        Some(Stackable::Float(data)) => println!("{:?}", data),
        None => (),
    }
}