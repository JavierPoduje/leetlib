#[derive(Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    pub fn from_vec(vec: &[i32]) -> Option<Box<ListNode>> {
        let mut result = None;

        for entry in vec.iter().rev() {
            let mut node = Self::new(*entry);
            node.next = result;
            result = Some(Box::new(node));
        }

        result
    }

    pub fn into_array(&self) -> Vec<i32> {
        let mut result = vec![self.val];
        let mut head = &self.next;

        while let Some(node) = head {
            result.push(node.val);
            head = &node.next;
        }

        result
    }
}
