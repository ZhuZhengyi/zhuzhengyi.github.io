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
    /// - 贪心法
    /// 1. 设max_pos: 当前能跳到的最远位置;
    /// 2. 从左至右依次遍历, 计算每个位置能跳到的最远位置;
    /// 3. 计算前, 判断是否能到达当前位置;
    /// 4. 如果最后能到达最后位置
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let l = nums.len();
        let mut max_pos = 0; //当前位置最远能跳到的位置
        for i in 0..l - 1 {
            // 如果当前位置在之前最远能到达的位置之外
            if max_pos < i {
                break; //则退出
            }
            // 更新到现在位置能到达的最大位置
            max_pos = max_pos.max(i + nums[i] as usize);
        }

        max_pos >= l - 1
    }
}
// @lc code=end
