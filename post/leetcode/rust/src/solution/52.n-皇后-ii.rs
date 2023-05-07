/*
 * @lc app=leetcode.cn id=52 lang=rust
 *
 * [52] N 皇后 II
 *
 * https://leetcode.cn/problems/n-queens-ii/description/
 *
 * algorithms
 * Hard (82.42%)
 * Likes:    429
 * Dislikes: 0
 * Total Accepted:    118.6K
 * Total Submissions: 143.8K
 * Testcase Example:  '4'
 *
 * n 皇后问题 研究的是如何将 n 个皇后放置在 n × n 的棋盘上，并且使皇后彼此之间不能相互攻击。
 *
 * 给你一个整数 n ，返回 n 皇后问题 不同的解决方案的数量。
 *
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 4
 * 输出：2
 * 解释：如上图所示，4 皇后问题存在两个不同的解法。
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 1
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= n <= 9
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        /// - 深度优先搜索回溯
        fn dfs(n: usize, board: &mut Vec<usize>, states: (i32, i32, i32), res: &mut i32) {
            let (col_mask, diagonal_45, diagonal_135) = states;
            // 所有行(n行)都已放置了'Q'
            if col_mask == (1 << n) - 1 {
                *res += 1;
                return; //结束本轮
            }

            // 依次尝试当前行的各个列位置
            for i in 0..n {
                let bit_info = 1 << i; // 下在当前行的第i列

                // 如果当前行所在的列,两条对角线已经有'Q'了, 则跳过当前位置
                if bit_info & (col_mask | diagonal_45 | diagonal_135) != 0 {
                    continue;
                }

                board.push(i); //在当前行第i列放置一个'Q'

                // 继续试探下一步下法
                dfs(
                    n,
                    board,
                    (
                        (col_mask | bit_info),
                        (diagonal_45 | bit_info) << 1, //整体往右下移一位
                        (diagonal_135 | bit_info) >> 1, //整体往左下移一位
                    ),
                    res,
                );

                // 撤消当前位置的棋(以尝试下一个位置)
                board.pop();
            }
        }

        let mut res = 0;
        dfs(n as usize, &mut vec![], (0, 0, 0), &mut res);

        res
    }
}
// @lc code=end
