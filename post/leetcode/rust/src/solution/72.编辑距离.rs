/*
 * @lc app=leetcode.cn id=72 lang=rust
 *
 * [72] 编辑距离
 *
 * https://leetcode.cn/problems/edit-distance/description/
 *
 * algorithms
 * Hard (62.81%)
 * Likes:    3012
 * Dislikes: 0
 * Total Accepted:    375.7K
 * Total Submissions: 598.1K
 * Testcase Example:  '"horse"\n"ros"'
 *
 * 给你两个单词 word1 和 word2， 请返回将 word1 转换成 word2 所使用的最少操作数  。
 *
 * 你可以对一个单词进行如下三种操作：
 *
 *
 * 插入一个字符
 * 删除一个字符
 * 替换一个字符
 *
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：word1 = "horse", word2 = "ros"
 * 输出：3
 * 解释：
 * horse -> rorse (将 'h' 替换为 'r')
 * rorse -> rose (删除 'r')
 * rose -> ros (删除 'e')
 *
 *
 * 示例 2：
 *
 *
 * 输入：word1 = "intention", word2 = "execution"
 * 输出：5
 * 解释：
 * intention -> inention (删除 't')
 * inention -> enention (将 'i' 替换为 'e')
 * enention -> exention (将 'n' 替换为 'x')
 * exention -> exection (将 'n' 替换为 'c')
 * exection -> execution (插入 'u')
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= word1.length, word2.length <= 500
 * word1 和 word2 由小写英文字母组成
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 dp[i][j]: 表示word1[0..i]和word2[0..j]的最小编辑距离;
    /// 2. 如果 word1[i] == word2[j], 则不用编辑, 有:
    ///    dp[i][j] = dp[i-1][j-1]
    /// 3. 如果 word1[i] != word2[j], 则
    ///    dp[i][j] = min(dp[i-1][j], dp[i][j-1], dp[i-1][j-1]) + 1
    ///    其中:
    ///     - dp[i-1][j]: 删除word1[i];
    ///     - dp[i][j-1]: 删除word2[j];
    ///     - dp[i-1][j-1]:
    ///  4. 初始条件:
    ///     dp[i][0] = i
    ///     dp[0][j] = j
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = vec![vec![0_i32; word2.len() + 1]; word1.len() + 1];
        for i in 1..=word1.len() {
            dp[i][0] = i as i32;
        }
        for j in 1..=word2.len() {
            dp[0][j] = j as i32;
        }
        for (i, w1) in word1.bytes().enumerate() {
            for (j, w2) in word2.bytes().enumerate() {
                if w1 == w2 {
                    dp[i + 1][j + 1] = dp[i][j];
                } else {
                    dp[i + 1][j + 1] =
                        std::cmp::min(std::cmp::min(dp[i][j + 1], dp[i + 1][j]), dp[i][j]) + 1;
                }
            }
        }
        dp[word1.len()][word2.len()]
    }
}
// @lc code=end

struct Solution;
