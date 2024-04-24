#[derive(Debug)]
struct Stack<T, const N: usize> {
    data: [T; N],
    top: usize,
}

impl<T: Default, const N: usize> Stack<T, N> {
    fn new() -> Self {
        Stack {
            data: [T::default(); N],
            top: 0,
        }
    }
    fn push(&mut self, item: i32) -> Result<(), &'static str>{
        if self.top == N {
            return Err("Stack is full");
        }
        self.data[self.top] = item;
        self.top += 1;
        Ok(())
    }
    fn pop(&mut self) -> Result<i32, &'static str> {
        if self.top == 0 {
            return Err("Stack is empty");
        }
        self.top -= 1;
        Ok(self.data[self.top])
    }
}
fn main() {
    let mut stack = Stack::<10>::new();
    for i in 0..10 {
        stack.push(i);
    }
    for _ in 0..10 {
        println!("{:?}", stack.pop());
    }
}