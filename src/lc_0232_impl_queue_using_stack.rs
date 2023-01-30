#[allow(dead_code)]
struct MyQueue {
    stack1: Vec<i32>,
    stack2: Vec<i32>,
}

impl MyQueue {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            stack1: vec![],
            stack2: vec![],
        }
    }

    #[allow(dead_code)]
    fn push(&mut self, x: i32) {
        while let Some(val) = self.stack1.pop() {
            self.stack2.push(val);
        }
        self.stack1.push(x);
        while let Some(val) = self.stack2.pop() {
            self.stack1.push(val);
        }
    }

    #[allow(dead_code)]
    fn pop(&mut self) -> i32 {
        self.stack1.pop().unwrap()
    }

    #[allow(dead_code)]
    fn peek(&self) -> i32 {
        *self.stack1.last().unwrap()
    }

    #[allow(dead_code)]
    fn empty(&self) -> bool {
        self.stack1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        q.peek();
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert!(!q.empty());
    }
}
