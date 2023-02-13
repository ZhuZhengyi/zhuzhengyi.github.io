// solution.rs
//

#[path = "1.两数之和.rs"]
mod _0001_two_sum;

#[path = "./10.正则表达式匹配.rs"]
mod _0010_is_match;

#[path = "./878.第-n-个神奇数字.rs"]
mod _0878_is_match;

// #[path="./102.二叉树的层序遍历.rs"]
// mod _0102_bt_level_travel;

/// Solution
pub struct Solution {}

impl Solution {
    /// new a Solution
    pub(crate) fn new() -> Self {
        Solution {}
    }
}

impl Default for Solution {
    fn default() -> Self {
        Self::new()
    }
}
