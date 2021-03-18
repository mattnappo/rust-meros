pub const DATADIR: &str = "./data/";

/// A generic LIFO collection.
pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    /// Construct a new stack.
    fn new() -> Self {
        Stack(Vec::new())
    }

    /// Construct a new stack given a vector.
    fn from_vec(vec: Vec<T>) -> Self {
        Stack(vec)
    }

    /// Push an item onto the stack.
    fn push(&mut self, item: T) {
        self.0.push(item);
    }

    /// Pop an item off the stack.
    fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
}
