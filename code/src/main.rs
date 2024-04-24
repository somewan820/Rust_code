#[derive(Debug)]
struct Stack<T, const N: usize> {
    data: [T; N],
    top: usize,
}

impl<T: Default, const N: usize> Stack<T, N> {
    fn new() -> Self {
        Stack {
            data: std::array::from_fn(|_i| T::default()),
            top: 0,
        }
    }
    fn push(&mut self, item: T) -> Result<(), &'static str>{
        if self.top == N {
            return Err("Stack is full");
        }
        self.data[self.top] = item;
        self.top += 1;
        Ok(())
    }
    fn pop(&mut self) -> Result<T, &'static str> {
        if self.top == 0 {
            return Err("Stack is empty");
        }
        self.top -= 1;
        let mut k = T::default();
        std::mem::swap(&mut k, &mut self.data[self.top]);
        Ok(k)
    }
}
fn main() {
    let mut stack = Stack::<i32, 10>::new();
    for i in 0..10 {
        stack.push(i);
    }
    for _ in 0..10 {
        println!("{:?}", stack.pop());
    }
}