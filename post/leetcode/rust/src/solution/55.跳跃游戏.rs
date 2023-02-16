/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] 跳跃游戏
 *
 * https://leetcode-cn.com/problems/jump-game/description/
 *
 * algorithms
 * Medium (38.11%)
 * Likes:    469
 * Dislikes: 0
 * Total Accepted:    61.2K
 * Total Submissions: 160.7K
 * Testcase Example:  '[2,3,1,1,4]'
 *
 * 给定一个非负整数数组，你最初位于数组的第一个位置。
 * 
 * 数组中的每个元素代表你在该位置可以跳跃的最大长度。
 * 
 * 判断你是否能够到达最后一个位置。
 * 
 * 示例 1:
 * 
 * 输入: [2,3,1,1,4]
 * 输出: true
 * 解释: 我们可以先跳 1 步，从位置 0 到达 位置 1, 然后再从位置 1 跳 3 步到达最后一个位置。
 * 
 * 
 * 示例 2:
 * 
 * 输入: [3,2,1,0,4]
 * 输出: false
 * 解释: 无论怎样，你总会到达索引为 3 的位置。但该位置的最大跳跃长度是 0 ， 所以你永远不可能到达最后一个位置。
 * 
 * 
 */

use super::*;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// * 贪心法
    /// 1. 依次检查当前能跳到的最远位置，如果该位置
    /// 
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let l = nums.len();
        let mut max_pos = 0;
        for i in 0..l-1 {
            max_pos = max_pos.max(i + nums[i] as usize);
            if max_pos <= i {
                break;
            }
        }

        max_pos >= l-1
    }
}
// @lc code=end

