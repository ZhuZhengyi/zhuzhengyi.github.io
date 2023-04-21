/*
 * @lc app=leetcode.cn id=130 lang=rust
 *
 * [130] 被围绕的区域
 *
 * https://leetcode.cn/problems/surrounded-regions/description/
 *
 * algorithms
 * Medium (46.23%)
 * Likes:    942
 * Dislikes: 0
 * Total Accepted:    220.3K
 * Total Submissions: 476.7K
 * Testcase Example:  '[["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]'
 *
 * 给你一个 m x n 的矩阵 board ，由若干字符 'X' 和 'O' ，找到所有被 'X' 围绕的区域，并将这些区域里所有的 'O' 用 'X'
 * 填充。
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：board =
 * [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
 * 输出：[["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
 * 解释：被围绕的区间不会存在于边界上，换句话说，任何边界上的 'O' 都不会被填充为 'X'。 任何不在边界上，或不与边界上的 'O' 相连的 'O'
 * 最终都会被填充为 'X'。如果两个元素在水平或垂直方向相邻，则称它们是“相连”的。
 *
 *
 * 示例 2：
 *
 *
 * 输入：board = [["X"]]
 * 输出：[["X"]]
 *
 *
 *
 *
 * 提示：
 *
 *
 * m == board.length
 * n == board[i].length
 * 1
 * board[i][j] 为 'X' 或 'O'
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - dfs
    /// 1. 从4条边界开始,将边上'O'及相连的'O'进行标记;
    /// 2. 按顺序重新遍历board, 将所有未标记的字符都标记为'X';
    /// 3. 将所有标记字符恢复为'O';
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board.first().unwrap_or(&vec![]).len();

        let mut stack = vec![];
        for (r, c) in (0..n)
            .map(|c| (0, c))
            .chain((0..n).map(|c| (m - 1, c)))
            .chain((1..m - 1).map(|r| (r, 0)))
            .chain((1..m - 1).map(|r| (r, n - 1)))
        {
            if board[r][c] == 'O' {
                stack.push((r, c));

                while let Some((r, c)) = stack.pop() {
                    // 将该点的前后左右四个元素都
                    for (ra, ca) in [(!0, 0), (1, 0), (0, !0), (0, 1)] {
                        let (rp, cp) = (r.wrapping_add(ra), c.wrapping_add(ca));
                        if rp < m && cp < n && board[rp][cp] == 'O' {
                            stack.push((rp, cp));
                        }
                    }

                    board[r][c] = 'M';
                }
            }
        }

        // 恢复标记
        board
            .iter_mut()
            .map(|row| {
                row.iter_mut()
                    .for_each(|e| *e = if *e == 'M' { 'O' } else { 'X' })
            })
            .for_each(drop)
    }
}
// @lc code=end

struct Solution;
