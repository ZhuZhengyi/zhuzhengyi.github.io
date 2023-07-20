/*
 * @lc app=leetcode.cn id=77 lang=rust
 *
 * [77] 组合
 *
 * https://leetcode.cn/problems/combinations/description/
 *
 * algorithms
 * Medium (77.11%)
 * Likes:    1428
 * Dislikes: 0
 * Total Accepted:    559K
 * Total Submissions: 725.2K
 * Testcase Example:  '4\n2'
 *
 * 给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。
 *
 * 你可以按 任何顺序 返回答案。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：n = 4, k = 2
 * 输出：
 * [
 * ⁠ [2,4],
 * ⁠ [3,4],
 * ⁠ [2,3],
 * ⁠ [1,2],
 * ⁠ [1,3],
 * ⁠ [1,4],
 * ]
 *
 * 示例 2：
 *
 *
 * 输入：n = 1, k = 1
 * 输出：[[1]]
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * 1
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 回溯法
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn backtrace(n: i32, k: i32, start: i32, tmp: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if tmp.len() as i32 == k {
                res.push(tmp.clone());
                return;
            }
            for i in start..=n {
                tmp.push(i as i32);
                backtrace(n, k, i + 1, tmp, res);
                tmp.pop();
            }
        }

        let mut res = vec![];
        let mut tmp = vec![];

        backtrace(n, k, 1, &mut tmp, &mut res);

        res
    }
}
// @lc code=end

struct Solution;
