/*
 * @lc app=leetcode.cn id=59 lang=rust
 *
 * [59] 螺旋矩阵 II
 *
 * https://leetcode.cn/problems/spiral-matrix-ii/description/
 *
 * algorithms
 * Medium (73.28%)
 * Likes:    1050
 * Dislikes: 0
 * Total Accepted:    315.1K
 * Total Submissions: 430.7K
 * Testcase Example:  '3'
 *
 * 给你一个正整数 n ，生成一个包含 1 到 n^2 所有元素，且元素按顺时针顺序螺旋排列的 n x n 正方形矩阵 matrix 。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 3
 * 输出：[[1,2,3],[8,9,4],[7,6,5]]
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 1
 * 输出：[[1]]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let (mut left, mut right, mut top, mut bottom) = (0, n - 1, 0, n - 1);
        let mut res = vec![vec![0; n]; n];
        let mut v = 1;
        while left <= right && top <= bottom {
            // 左->右
            for col in left..=right {
                res[top][col] = v;
                v += 1;
            }
            if top < bottom {
                top += 1;
            } else {
                break;
            }

            // 上->下
            for row in top..=bottom {
                res[row][right] = v;
                v += 1;
            }
            if right > left {
                right -= 1;
            } else {
                break;
            }

            // 右->左
            for col in (left..=right).rev() {
                res[bottom][col] = v;
                v += 1;
            }
            if bottom > top {
                bottom -= 1;
            } else {
                break;
            }

            // 下->上
            for row in (top..=bottom).rev() {
                res[row][left] = v;
                v += 1;
            }
            if left < right {
                left += 1;
            } else {
                break;
            }
        }

        res
    }
}
// @lc code=end
