// use std::collections::VecDeque;

// #[derive(Debug, Clone)]
// pub struct TreeNode<T> {
//     pub value: T,
//     pub left: Option<Box<TreeNode<T>>>,
//     pub right: Option<Box<TreeNode<T>>>,    
// }

// impl<T> TreeNode<T>{
//     pub fn new(value: T) -> Option<Box<TreeNode<T>>>{
//        Some(Box::new(TreeNode {
//                value,
//                left: None,
//                right: None,
//             }))
//     }
// }

// pub fn build_binary_tree_from_array<T>(array: Vec<Option<T>>) -> Option<Box<TreeNode<T>>> where T: Clone, {
//     let mut root: Option<Box<TreeNode<T>>> = None;
//     let mut queue: VecDeque<(Option<Box<TreeNode<T>>>, usize)> = VecDeque::new();

//     if let Some(root_val) = array.get(0).cloned().flatten() {
//         root = Some(Box::new(TreeNode::new(root_val.clone())));

//         queue.push_back((root.as_mut().map(|r| r.as_mut()), 0));

//         while let Some((mut node, index)) = queue.pop_front() {
//             let left_index = 2 * index + 1;
//             let right_index = 2 * index + 2;
        
//             if let Some(left_val) = array.get(left_index).cloned().flatten() {
//                 let left_child: Option<Box<TreeNode<T>>> = Some(Box::new(TreeNode::new(left_val.clone())));
//                 node.as_mut().unwrap().left = left_child.clone();
//                 queue.push_back((left_child, left_index));
//             }
            
//             if let Some(right_val) = array.get(right_index).cloned().flatten() {
//                 let right_child: Option<Box<TreeNode<T>>> = Some(Box::new(TreeNode::new(right_val.clone())));
//                 node.as_mut().unwrap().right = right_child.clone();
//                 queue.push_back((right_child, right_index));
//             }
//         }
                    
//     }
// }