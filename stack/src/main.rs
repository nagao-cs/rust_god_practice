struct Stack {
    top: usize, 
    array: [i32; 10],
}

impl Stack {
    fn push(&mut self, data: i32) {
        if self.top < 10 {
            self.array[self.top] = data;
            self.top += 1;
        }
        else {
            println!("Stack is full");
        }
    }

    fn pop(&mut self) -> i32 {
        if self.top > 0 {
            self.top -= 1;
            return self.array[self.top];
        }
        else {
            println!("Stack is empty");
            return -1;
        }
    }

    fn new() -> Stack {
        Stack {
            top: 0,
            array: [0; 10]
        }
    }
}

fn main() {
    let mut stack_a = Stack::new();
    stack_a.push(1);
    stack_a.push(2);
    stack_a.push(3);
    println!("{}", stack_a.pop());
    println!("{}", stack_a.pop());
    println!("{}", stack_a.pop());
    println!("{}", stack_a.pop());
}