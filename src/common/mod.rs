pub const DATADIR: &str = "./data/";

/// A generic LIFO collection.
pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    /// Construct a new stack.
    pub fn new() -> Self {
        Stack(Vec::new())
    }

    /// Construct a new stack given a vector.
    pub fn from_vec(vec: Vec<T>) -> Self {
        Stack(vec)
    }

    /// Push an item onto the stack.
    pub fn push(&mut self, item: T) {
        self.0.push(item);
    }

    /// Pop an item off the stack.
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    // View the internal vector as a reference.
    pub fn vec(&self) -> &Vec<T> {
        &self.0
    }
}
