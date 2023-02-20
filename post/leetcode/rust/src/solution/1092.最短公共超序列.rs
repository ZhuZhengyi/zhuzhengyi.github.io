/*
 * @lc app=leetcode.cn id=1092 lang=rust
 *
 * [1092] 最短公共超序列
 *
 * https://leetcode.cn/problems/shortest-common-supersequence/description/
 *
 * algorithms
 * Hard (54.17%)
 * Likes:    114
 * Dislikes: 0
 * Total Accepted:    5.2K
 * Total Submissions: 9.6K
 * Testcase Example:  '"abac"\n"cab"'
 *
 * 给出两个字符串 str1 和 str2，返回同时以 str1 和 str2
 * 作为子序列的最短字符串。如果答案不止一个，则可以返回满足条件的任意一个答案。
 *
 * （如果从字符串 T 中删除一些字符（也可能不删除，并且选出的这些字符可以位于 T 中的 任意位置），可以得到字符串 S，那么 S 就是 T
 * 的子序列）
 *
 *
 *
 * 示例：
 *
 * 输入：str1 = "abac", str2 = "cab"
 * 输出："cabac"
 * 解释：
 * str1 = "abac" 是 "cabac" 的一个子串，因为我们可以删去 "cabac" 的第一个 "c"得到 "abac"。
 * str2 = "cab" 是 "cabac" 的一个子串，因为我们可以删去 "cabac" 末尾的 "ac" 得到 "cab"。
 * 最终我们给出的答案是满足上述属性的最短字符串。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= str1.length, str2.length <= 1000
 * str1 和 str2 都由小写英文字母组成。
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        todo!()
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
            Solution::shortest_common_supersequence("abac".into(), "cab".into()),
            "cabac".to_string()
        );
    }
}
