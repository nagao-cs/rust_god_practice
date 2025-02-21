pub struct Stack<T: Clone> {
    top: usize,
    size: usize, 
    stack: Vec<Option<T>>,
}

impl<T: Clone> Stack<T> {
    pub fn new(size: usize) -> Stack<T> {
        Stack {
            top: 0,
            size,
            stack: vec![None; size],
        }
    }

    pub fn push(&mut self, data: T) {
        if self.is_full() {
            println!("Stack is full");
            return;
        }
        self.stack[self.top] = Some(data);
        self.top += 1;
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        if self.is_empty() {
            return Err("Stack is empty");
        }
        self.top -= 1;
        self.stack[self.top].take().ok_or("Stack is empty")
    }

    fn is_full(&self) -> bool {
        self.top == self.size
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }
}
