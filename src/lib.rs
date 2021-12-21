use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn tree_node_wrap(node: TreeNode) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn new_left_right(val: i32, left: i32, right: i32) -> Self {
        TreeNode {
            val,
            left: Self::tree_node_wrap(Self::new(left)),
            right: Self::tree_node_wrap(Self::new(right)),
        }
    }

    pub fn new_left(val: i32, left: i32) -> Self {
        TreeNode {
            val,
            left: Self::tree_node_wrap(Self::new(left)),
            right: None,
        }
    }

    pub fn new_right(val: i32, right: i32) -> Self {
        let right = Self::new(right);
        TreeNode {
            val,
            left: None,
            right: Some(Rc::new(RefCell::new(right))),
        }
    }
}

/*
  - unique BST structures
    - should produce the same sequence when
      traversed inorder
    - if the sequnce is sorted ascendingly
      the left child will have the smaller
      numbers and the right child will have
      the larger number when compared to the
      root node
*/

pub struct Solution {}

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let nums: Vec<i32> = (1..n + 1).collect();
        Solution::count(&nums)
    }

    /*
      - every one in the vec can be the root
        - this is the key
      - the smaller numers have to be in the
        left child
      - the larger numbers have to be in the
        right child
    */

    fn count(nums: &Vec<i32>) -> i32 {
        if *nums == vec![] {
            /*
              - no child also counts as one possible struct
              - leaf node is such a case where there is
                no left or right child for it. Therefore,
                the total number of possible constructs
                is left x right = 1 x 1 = 1
            */
            return 1;
        }

        let mut total_count = 0;

        /*
          - everyone in the nums takes turn
            to be the root node
        */
        for (index, _) in nums.iter().enumerate() {
            /*
              - split the vec to left and right
              - left vec has the smaller numbers
                comparing to the root
              - right vec has the larger numbers
                comparing to the root
            */
            let (left, right) = nums.split_at(index);
            let left_count = Solution::count(&left.to_vec());
            let right_count = if let Some((_, elements)) = right.split_first() {
                Solution::count(&elements.to_vec())
            } else {
                /*
                  - no more right child, we are at a leaf node
                  - why is that? this is because the nums is
                    sorted from small to large numbers, and the
                    way we split it left child will always be
                    empty first than the right child. so when
                    the right child is also empty we are at
                    a leaf node already
                */
                1
            };
            total_count += left_count * right_count;
        }

        total_count
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        assert_eq!(Solution::num_trees(3), 5);
    }

    #[test]
    fn sample_2() {
        assert_eq!(Solution::num_trees(1), 1);
    }
    #[test]
    fn sample_3() {}
}
