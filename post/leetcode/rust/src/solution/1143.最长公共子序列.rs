/*
 * @lc app=leetcode.cn id=1143 lang=rust
 *
 * [1143] 最长公共子序列
 *
 * https://leetcode.cn/problems/longest-common-subsequence/description/
 *
 * algorithms
 * Medium (64.96%)
 * Likes:    1215
 * Dislikes: 0
 * Total Accepted:    306.5K
 * Total Submissions: 471.8K
 * Testcase Example:  '"abcde"\n"ace"'
 *
 * 给定两个字符串 text1 和 text2，返回这两个字符串的最长 公共子序列 的长度。如果不存在 公共子序列 ，返回 0 。
 *
 * 一个字符串的 子序列
 * 是指这样一个新的字符串：它是由原字符串在不改变字符的相对顺序的情况下删除某些字符（也可以不删除任何字符）后组成的新字符串。
 *
 *
 * 例如，"ace" 是 "abcde" 的子序列，但 "aec" 不是 "abcde" 的子序列。
 *
 *
 * 两个字符串的 公共子序列 是这两个字符串所共同拥有的子序列。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：text1 = "abcde", text2 = "ace"
 * 输出：3
 * 解释：最长公共子序列是 "ace" ，它的长度为 3 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：text1 = "abc", text2 = "abc"
 * 输出：3
 * 解释：最长公共子序列是 "abc" ，它的长度为 3 。
 *
 *
 * 示例 3：
 *
 *
 * 输入：text1 = "abc", text2 = "def"
 * 输出：0
 * 解释：两个字符串没有公共子序列，返回 0 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1
 * text1 和 text2 仅由小写英文字符组成。
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 dp[i][j]: 以text1[0..i], text2[0..j]的最长公共子序列
    /// 2. 满足如下条件：
    ///     dp[0][j] = 0
    ///     dp[i][0] = 0
    /// 3. dp[i][j] = dp[i-1][j-1] + 1 (text1[i] == text2[j])
    ///           或者 = max(dp[i-1][j], dp[i][j-1])
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (m, n) = (text1.len(), text2.len());
        let mut dp = vec![vec![0; n + 1]; m + 1];
        let mut longest = 0;
        for i in 1..(m + 1) {
            for j in 1..(n + 1) {
                if text1.chars().nth(i - 1) == text2.chars().nth(j - 1) {
                    dp[i][j] = dp[i - 1][j - 1] + 1
                } else {
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
                }
                longest = std::cmp::max(longest, dp[i][j]);
            }
        }

        longest
    }
}
// @lc code=end
use super::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".into(), "ace".into()),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".into(), "abc".into()),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".into(), "def".into()),
            0
        );
    }
}
