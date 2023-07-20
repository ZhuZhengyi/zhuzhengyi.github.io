/*
 * @lc app=leetcode.cn id=85 lang=rust
 *
 * [85] 最大矩形
 *
 * https://leetcode.cn/problems/maximal-rectangle/description/
 *
 * algorithms
 * Hard (54.69%)
 * Likes:    1547
 * Dislikes: 0
 * Total Accepted:    178.6K
 * Total Submissions: 326.5K
 * Testcase Example:  '[["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]'
 *
 * 给定一个仅包含 0 和 1 、大小为 rows x cols 的二维二进制矩阵，找出只包含 1 的最大矩形，并返回其面积。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：matrix =
 * [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
 * 输出：6
 * 解释：最大矩形如上图所示。
 *
 *
 * 示例 2：
 *
 *
 * 输入：matrix = []
 * 输出：0
 *
 *
 * 示例 3：
 *
 *
 * 输入：matrix = [["0"]]
 * 输出：0
 *
 *
 * 示例 4：
 *
 *
 * 输入：matrix = [["1"]]
 * 输出：1
 *
 *
 * 示例 5：
 *
 *
 * 输入：matrix = [["0","0"]]
 * 输出：0
 *
 *
 *
 *
 * 提示：
 *
 *
 * rows == matrix.length
 * cols == matrix[0].length
 * 1 <= row, cols <= 200
 * matrix[i][j] 为 '0' 或 '1'
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 令 heights[i][j]: matrix[i][j]从上到下连续'1'的个数(高度);
    /// 2. 如果 matrix[i][j] == '0', => heights[i][j] = 0;
    ///        matrix[i][j] == '1', => heights[i][j] = heights[i-1][j] + 1;
    /// 3. 令 dp[i][j][k]: 表示以matrix[i][j]为右下角的,高度为k的最大矩形面积;
    ///    则 matrix[i][j] == '0' => dp[i][j][k] = 0;
    ///       matrix[i][j] == '1' => dp[i][j][k] = dp[i][j-1][k] + k;
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut heights = vec![vec![0; n + 1]; m + 1]; // matrix[i][j]从上到下连续1的个数
        let mut dp = vec![vec![vec![0_i32; m + 1]; n + 1]; m + 1]; //

        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == '0' {
                    continue;
                }
                heights[i + 1][j + 1] = heights[i][j + 1] + 1;
                for k in 1..=heights[i + 1][j + 1] {
                    dp[i + 1][j + 1][k] = dp[i + 1][j][k] + (k as i32);
                    res = res.max(dp[i + 1][j + 1][k]);
                }
            }
        }

        res
    }
}
// @lc code=end

struct Solution;
