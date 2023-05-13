/*
 * @lc app=leetcode.cn id=64 lang=rust
 *
 * [64] 最小路径和
 *
 * https://leetcode.cn/problems/minimum-path-sum/description/
 *
 * algorithms
 * Medium (69.48%)
 * Likes:    1500
 * Dislikes: 0
 * Total Accepted:    482K
 * Total Submissions: 693.6K
 * Testcase Example:  '[[1,3,1],[1,5,1],[4,2,1]]'
 *
 * 给定一个包含非负整数的 m x n 网格 grid ，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。
 *
 * 说明：每次只能向下或者向右移动一步。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：grid = [[1,3,1],[1,5,1],[4,2,1]]
 * 输出：7
 * 解释：因为路径 1→3→1→1→1 的总和最小。
 *
 *
 * 示例 2：
 *
 *
 * 输入：grid = [[1,2,3],[4,5,6]]
 * 输出：12
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == grid.length
 * n == grid[i].length
 * 1
 * 0
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设dp[m][n]: mxn网格最小路径和;
    /// 2. 递推公式: dp[i][j] = min(dp[i-1][j], dp[i][j-1]) + grid[i][j]
    /// 3. 初始条件: dp[0][0] = grid[0][0];
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![std::i32::MAX; n + 1]; m + 1];
        dp[1][0] = 0;
        dp[0][1] = 0;
        for i in 0..m {
            for j in 0..n {
                dp[i + 1][j + 1] = std::cmp::min(dp[i][j + 1], dp[i + 1][j]) + grid[i][j];
            }
        }

        dp[m][n]
    }
}
// @lc code=end
