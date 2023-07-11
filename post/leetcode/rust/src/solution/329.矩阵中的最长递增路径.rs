/*
 * @lc app=leetcode.cn id=329 lang=rust
 *
 * [329] 矩阵中的最长递增路径
 *
 * https://leetcode.cn/problems/longest-increasing-path-in-a-matrix/description/
 *
 * algorithms
 * Hard (51.76%)
 * Likes:    774
 * Dislikes: 0
 * Total Accepted:    97.9K
 * Total Submissions: 189K
 * Testcase Example:  '[[9,9,4],[6,6,8],[2,1,1]]'
 *
 * 给定一个 m x n 整数矩阵 matrix ，找出其中 最长递增路径 的长度。
 *
 * 对于每个单元格，你可以往上，下，左，右四个方向移动。 你 不能 在 对角线 方向上移动或移动到 边界外（即不允许环绕）。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：matrix = [[9,9,4],[6,6,8],[2,1,1]]
 * 输出：4
 * 解释：最长递增路径为 [1, 2, 6, 9]。
 *
 * 示例 2：
 *
 *
 * 输入：matrix = [[3,4,5],[3,2,6],[2,2,1]]
 * 输出：4
 * 解释：最长递增路径是 [3, 4, 5, 6]。注意不允许在对角线方向上移动。
 *
 *
 * 示例 3：
 *
 *
 * 输入：matrix = [[1]]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == matrix.length
 * n == matrix[i].length
 * 1
 * 0
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - dfs
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        //
        fn dfs(matrix: &[Vec<i32>], step: (usize, usize), stat: &mut Vec<Vec<Option<i32>>>) -> i32 {
            // 如果该点遍历过, 则直接返回;
            if let Some(c) = stat[step.0][step.1] {
                return c;
            }
            let mut ret = 1;
            // 从该点开始, 前后左右四个方向递归遍历
            for d in [0, -1, 0, 1, 0].windows(2) {
                let (ni, nj) = ((step.0 as i32) + d[0], (step.1 as i32) + d[1]);
                let (ni, nj) = (ni as usize, nj as usize);
                if (0..matrix.len()).contains(&ni) && (0..matrix[0].len()).contains(&nj) {
                    // 递增方向
                    if matrix[ni][nj] > matrix[step.0][step.1] {
                        // 当前坐标路径最大递增长度 为四个方向中最长的那个+1
                        ret = ret.max(dfs(matrix, (ni, nj), stat) + 1);
                    }
                }
            }
            // 记录该坐标最长递增路径
            stat[step.0][step.1] = Some(ret);

            ret
        }

        let mut res = 0;
        let mut stat = vec![vec![None; matrix[0].len()]; matrix.len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                // 所有点开始的最长递增路径 中 最大的
                res = res.max(dfs(&matrix, (i, j), &mut stat));
            }
        }

        res
    }
}
// @lc code=end

struct Solution;
