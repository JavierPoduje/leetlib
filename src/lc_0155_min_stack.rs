pub struct MinStack {
    stack: Vec<i32>,
    minstack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            minstack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        if let Some(last) = self.minstack.last() {
            self.minstack.push(val.min(*last));
        } else {
            self.minstack.push(val);
        }
    }

    pub fn pop(&mut self) {
        self.stack.pop();
        self.minstack.pop();
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.minstack.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1_test() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(-3, min_stack.get_min());
        min_stack.pop();
        assert_eq!(0, min_stack.top());
        assert_eq!(-2, min_stack.get_min());
    }
}
