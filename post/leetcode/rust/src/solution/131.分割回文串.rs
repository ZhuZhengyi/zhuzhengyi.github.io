/*
 * @lc app=leetcode.cn id=131 lang=rust
 *
 * [131] 分割回文串
 *
 * https://leetcode.cn/problems/palindrome-partitioning/description/
 *
 * algorithms
 * Medium (73.42%)
 * Likes:    1567
 * Dislikes: 0
 * Total Accepted:    305.8K
 * Total Submissions: 416.6K
 * Testcase Example:  '"aab"'
 *
 * 给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是 回文串 。返回 s 所有可能的分割方案。
 *
 * 回文串 是正着读和反着读都一样的字符串。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "aab"
 * 输出：[["a","a","b"],["aa","b"]]
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "a"
 * 输出：[["a"]]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * s 仅由小写英文字母组成
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 回溯法
    pub fn partition(s: String) -> Vec<Vec<String>> {
        fn is_huiwen(s: &[u8]) -> bool {
            let (mut l, mut r) = (0, s.len() - 1);
            while l < r {
                if s[l] != s[r] {
                    return false;
                } else {
                    l += 1;
                    r -= 1;
                }
            }
            return true;
        }
        fn backtrace(s: &str, step: usize, tmp: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
            if step == s.len() {
                res.push(tmp.to_vec());
                return;
            }

            for off in step + 1..=s.len() {
                if !is_huiwen(s[step..off].as_bytes()) {
                    continue;
                }
                tmp.push(s[step..off].to_string());
                backtrace(s, off, tmp, res);
                tmp.pop();
            }
        }
        let mut res = vec![];
        let mut tmp = vec![];

        backtrace(&s, 0, &mut tmp, &mut res);

        res
    }
}
// @lc code=end

struct Solution;
