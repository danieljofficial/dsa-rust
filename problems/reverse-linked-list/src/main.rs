fn main() {
    type Link = Option<Box<ListNode>>;
    struct ListNode {
        val: i32,
        next_node: Link,
    }

    impl ListNode {
        fn new(val: i32) -> Self {
            ListNode {
                val,
                next_node: None,
            }
        }
    }

    fn add_two_numbers(l1: Link, l2: Link) -> Link {
        let mut result_head = ListNode::new(0);
        let mut current_node = &mut result_head;

        let mut list1 = &l1;
        let mut list2 = &l2;
        let mut carry = 0;

        while list1.is_some() || list2.is_some() || carry > 0 {
            let mut sum = carry;

            if let Some(node) = list1 {
                sum += node.val;
                list1 = &node.next_node
            }

            if let Some(node) = list2 {
                sum += node.val;
                list2 = &node.next_node
            }

            carry = sum / 10;
            let digit = sum % 10;

            current_node.next_node = Some(Box::new(ListNode::new(digit)));
            current_node = current_node.next_node.as_mut().unwrap();
        }

        result_head.next_node
    }
}
