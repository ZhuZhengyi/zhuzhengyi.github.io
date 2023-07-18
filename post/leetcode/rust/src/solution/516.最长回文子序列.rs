/*
 * @lc app=leetcode.cn id=516 lang=rust
 *
 * [516] 最长回文子序列
 *
 * https://leetcode.cn/problems/longest-palindromic-subsequence/description/
 *
 * algorithms
 * Medium (67.15%)
 * Likes:    1070
 * Dislikes: 0
 * Total Accepted:    188.2K
 * Total Submissions: 280.2K
 * Testcase Example:  '"bbbab"'
 *
 * 给你一个字符串 s ，找出其中最长的回文子序列，并返回该序列的长度。
 *
 * 子序列定义为：不改变剩余字符顺序的情况下，删除某些字符或者不删除任何字符形成的一个序列。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "bbbab"
 * 输出：4
 * 解释：一个可能的最长回文子序列为 "bbbb" 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "cbbd"
 * 输出：2
 * 解释：一个可能的最长回文子序列为 "bb" 。
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
    /// - 动态规划
    /// 1. 设 dp[i][j]: 为s[i..j]的最长回文子序列;
    /// 2. 递推关系:
    ///                   = dp[i-1][j-1] + 2 (s[i] == s[j])
    ///    如果: dp[i][j] or
    ///                   = max(dp[i+1][j-1], dp[i][j-1]) (s[i] != s[j])
    /// 3. 初始条件: dp[i][i] = 0
    /// 4. 目标: dp[0][n-1]
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let mut res = 1;
        let sb = s.as_bytes();
        let mut dp = vec![vec![0; sb.len()]; sb.len()];
        for i in (0..s.len() - 1).rev() {
            dp[i][i] = 1;
            for j in (i + 1)..s.len() {
                dp[i][j] = if sb[i] == sb[j] {
                    dp[i + 1][j - 1] + 2
                } else {
                    dp[i + 1][j].max(dp[i][j - 1])
                };
                res = res.max(dp[i][j]);
            }
        }

        res
    }
}
// @lc code=end

struct Solution;
