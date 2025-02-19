pub struct Stack {
    top: usize,
    size: usize, 
    stack: Vec<Option<i32>>,
}

impl Stack {
    pub fn push(&mut self, data: i32) {
        if self.is_full() {
            println!("Stack is full");
        }
        else {
            self.stack[self.top] = Some(data);
            // self.stack.push(Some(data));
            self.top += 1;
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.is_empty() {
            println!("Stack is empty");
            return None;
        }
        else {
            self.top -= 1;
            return self.stack[self.top];
        }
    }

    pub fn new(size: usize) -> Stack {
        Stack {
            top: 0,
            size: size,
            stack: vec![None; size],
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