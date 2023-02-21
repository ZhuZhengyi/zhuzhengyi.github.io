/*
 * @lc app=leetcode.cn id=45 lang=rust
 *
 * [45] 跳跃游戏 II
 *
 * https://leetcode-cn.com/problems/jump-game-ii/description/
 *
 * algorithms
 * Medium (43.11%)
 * Likes:    1270
 * Dislikes: 0
 * Total Accepted:    222K
 * Total Submissions: 513.5K
 * Testcase Example:  '[2,3,1,1,4]'
 *
 * 给你一个非负整数数组 nums ，你最初位于数组的第一个位置。
 *
 * 数组中的每个元素代表你在该位置可以跳跃的最大长度。
 *
 * 你的目标是使用最少的跳跃次数到达数组的最后一个位置。
 *
 * 假设你总是可以到达数组的最后一个位置。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: nums = [2,3,1,1,4]
 * 输出: 2
 * 解释: 跳到最后一个位置的最小跳跃数是 2。
 * 从下标为 0 跳到下标为 1 的位置，跳 1 步，然后跳 3 步到达数组的最后一个位置。
 *
 *
 * 示例 2:
 *
 *
 * 输入: nums = [2,3,0,1,4]
 * 输出: 2
 *
 *
 *
 *
 * 提示:
 *
 *
 * 1
 * 0
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// * 贪心法
    /// 1. cur_jump_max_pos：每跳能到达的最远位置；
    /// 2. fur_max_pos：每步能达到的最远位置
    /// 2. 如果当前步达到
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut step = 0; //已跳的步数
        let last_jump_max_pos = 0; //能跳到的最远位置
        let mut fur_max_pos = 0; //当前格能跳到的最远位置
        let mut cur_max_pos = 0; //当前格能跳到的最远位置
        for i in 0..nums.len() - 1 {
            // 记录当前可以跳到的最远的位置
            fur_max_pos = fur_max_pos.max(i + nums[i] as usize);
            if i == cur_max_pos {
                //到达当前步能跳到的最远位置，则必须跳下一步
                cur_max_pos = fur_max_pos; //更新当前步最远能跳的位置
                step += 1; //跳一步
            }
        }

        step as i32
    }
}
// @lc code=end
