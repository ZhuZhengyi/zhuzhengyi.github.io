/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 *
 * https://leetcode-cn.com/problems/longest-common-prefix/description/
 *
 * algorithms
 * Easy (41.07%)
 * Likes:    1831
 * Dislikes: 0
 * Total Accepted:    644K
 * Total Submissions: 1.6M
 * Testcase Example:  '["flower","flow","flight"]'
 *
 * 编写一个函数来查找字符串数组中的最长公共前缀。
 *
 * 如果不存在公共前缀，返回空字符串 ""。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：strs = ["flower","flow","flight"]
 * 输出："fl"
 *
 *
 * 示例 2：
 *
 *
 * 输入：strs = ["dog","racecar","car"]
 * 输出：""
 * 解释：输入不存在公共前缀。
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= strs.length <= 200
 * 0 <= strs[i].length <= 200
 * strs[i] 仅由小写英文字母组成
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 依次比较其他字符串各位字符和第一个字符串字符，记录相同的前缀；
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        match strs.is_empty() {
            true => "".to_string(),
            _ => strs.iter().skip(1).fold(strs[0].clone(), |acc, s| {
                acc.chars()
                    .zip(s.chars())
                    .take_while(|(c1, c2)| c1 == c2)
                    .map(|(c1, _)| c1)
                    .collect()
            }),
        }
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            "".to_string()
        );
    }
}
