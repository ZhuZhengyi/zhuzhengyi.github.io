/*
 * @lc app=leetcode.cn id=46 lang=rust
 *
 * [46] 全排列
 *
 * https://leetcode-cn.com/problems/permutations/description/
 *
 * algorithms
 * Medium (77.19%)
 * Likes:    1020
 * Dislikes: 0
 * Total Accepted:    222.9K
 * Total Submissions: 288.6K
 * Testcase Example:  '[1,2,3]'
 *
 * 给定一个 没有重复 数字的序列，返回其所有可能的全排列。
 * 
 * 示例:
 * 
 * 输入: [1,2,3]
 * 输出:
 * [
 * ⁠ [1,2,3],
 * ⁠ [1,3,2],
 * ⁠ [2,1,3],
 * ⁠ [2,3,1],
 * ⁠ [3,1,2],
 * ⁠ [3,2,1]
 * ]
 * 
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 回溯法
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {

    }

    fn dfs(nums: Vec<i32>, 
        acc: &mut Vec<i32>,
        res: &mut Vec<i32>, 
        visited: &mut HashSet<i32>
    ) {
        if acc.len() == nums.len() {
            
        }

    }
}
// @lc code=end

