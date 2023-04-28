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

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 回溯法+bitmap
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res = vec![vec![]];

        /// - 深度优先搜索回溯
        ///
        fn dfs(n: i32, mut tmp_queens: Vec<usize>, mut res: Vec<Vec<String>>) {
            // 终止条件
            if n == tmp_queens.len() as i32 {
                //
                let mut t = vec![];
                for i in tmp_queens {
                    let mut s = String::new();
                    t.push(s);
                }
                //
                res.push(t);
                return;
            }

            //
            for i in 0..n {
                if is_valid(n, i) {
                    tmp_queens.push(i as usize);
                    dfs(n, tmp_queens, res);
                    tmp_queens.pop();
                }
            }
        }

        // 
        fn is_valid(n: i32, tmp_q) -> bool {

        }

        dfs(n, vec![], res);

        res
    }
}
// @lc code=end
