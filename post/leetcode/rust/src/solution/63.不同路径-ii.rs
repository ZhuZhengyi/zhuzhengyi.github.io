/*
 * @lc app=leetcode.cn id=63 lang=rust
 *
 * [63] 不同路径 II
 *
 * https://leetcode.cn/problems/unique-paths-ii/description/
 *
 * algorithms
 * Medium (41.00%)
 * Likes:    1045
 * Dislikes: 0
 * Total Accepted:    370.1K
 * Total Submissions: 902.2K
 * Testcase Example:  '[[0,0,0],[0,1,0],[0,0,0]]'
 *
 * 一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。
 *
 * 机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish”）。
 *
 * 现在考虑网格中有障碍物。那么从左上角到右下角将会有多少条不同的路径？
 *
 * 网格中的障碍物和空位置分别用 1 和 0 来表示。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
 * 输出：2
 * 解释：3x3 网格的正中间有一个障碍物。
 * 从左上角到右下角一共有 2 条不同的路径：
 * 1. 向右 -> 向右 -> 向下 -> 向下
 * 2. 向下 -> 向下 -> 向右 -> 向右
 *
 *
 * 示例 2：
 *
 *
 * 输入：obstacleGrid = [[0,1],[0,0]]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == obstacleGrid.length
 * n == obstacleGrid[i].length
 * 1 <= m, n <= 100
 * obstacleGrid[i][j] 为 0 或 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设dp[m][n]: mxn网格的不同路径数;
    /// 2. 递推关系: dp[i][j] = 0 , obstacle_grid[i][j] == 1;
    ///                or    = dp[i-1][j] + dp[i][j-1], obstacle_grid[i][j] == 0;
    /// 3. 初始条件: dp[i][j] == 0,
    ///             dp[1][1] == 1,
    ///    注: 为统一处理边界情况,dp行列各增加一行,dp初始化分配[m+1][n+1]的空间;
    /// 4. 结果: dp[m][n]
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        for i in 1..(m + 1) {
            for j in 1..(n + 1) {
                dp[i][j] = if obstacle_grid[i - 1][j - 1] == 1 {
                    0
                } else if i == 1 && j == 1 {
                    1 - obstacle_grid[0][0]
                } else {
                    dp[i - 1][j] + dp[i][j - 1]
                }
            }
        }

        dp[m][n]
    }
}
// @lc code=end
