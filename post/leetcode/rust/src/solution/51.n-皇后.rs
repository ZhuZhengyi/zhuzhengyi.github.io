/*
 * @lc app=leetcode.cn id=51 lang=rust
 *
 * [51] N 皇后
 *
 * https://leetcode.cn/problems/n-queens/description/
 *
 * algorithms
 * Hard (74.17%)
 * Likes:    1577
 * Dislikes: 0
 * Total Accepted:    268.1K
 * Total Submissions: 361.4K
 * Testcase Example:  '4'
 *
 * 按照国际象棋的规则，皇后可以攻击与之处在同一行或同一列或同一斜线上的棋子。
 *
 * n 皇后问题 研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
 *
 * 给你一个整数 n ，返回所有不同的 n 皇后问题 的解决方案。
 *
 *
 *
 * 每一种解法包含一个不同的 n 皇后问题 的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 4
 * 输出：[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
 * 解释：如上图所示，4 皇后问题存在两个不同的解法。
 *
 *
 * 示例 2：
 *
 *
 * 输入：n = 1
 * 输出：[["Q"]]
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

use std::vec;

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 回溯法+bitmap
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        /// - 深度优先搜索回溯
        fn dfs(
            n: usize,
            board: &mut Vec<usize>,
            states: (i32, i32, i32),
            res: &mut Vec<Vec<String>>,
        ) {
            let (col_mask, diagonal_45, diagonal_135) = states;
            // 所有行(n行)都已放置了'Q'
            if col_mask == (1 << n) - 1 {
                res.push(decode(n, board)); //记录本轮棋谱
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

        /// 解码,将棋盘从Vec<usize>转化为Vec<String>
        fn decode(n: usize, board: &Vec<usize>) -> Vec<String> {
            use std::iter::FromIterator;

            board
                .iter()
                .enumerate()
                .fold(vec![vec!['.'; n]; n], |mut brd, (i, &j)| {
                    brd[i][j] = 'Q';
                    brd
                })
                .iter()
                .map(|x| String::from_iter(x))
                .collect()
        }

        let mut res = vec![];
        dfs(n as usize, &mut vec![], (0, 0, 0), &mut res);

        res
    }
}
// @lc code=end
