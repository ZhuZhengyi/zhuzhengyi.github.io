/*
 * @lc app=leetcode.cn id=796 lang=rust
 *
 * [796] 旋转字符串
 *
 * https://leetcode-cn.com/problems/rotate-string/description/
 *
 * algorithms
 * Easy (50.90%)
 * Likes:    101
 * Dislikes: 0
 * Total Accepted:    13.6K
 * Total Submissions: 26.7K
 * Testcase Example:  '"abcde"\n"cdeab"'
 *
 * 给定两个字符串, A 和 B。
 *
 * A 的旋转操作就是将 A 最左边的字符移动到最右边。 例如, 若 A = 'abcde'，在移动一次之后结果就是'bcdea'
 * 。如果在若干次旋转操作之后，A 能变成B，那么返回True。
 *
 *
 * 示例 1:
 * 输入: A = 'abcde', B = 'cdeab'
 * 输出: true
 *
 * 示例 2:
 * 输入: A = 'abcde', B = 'abced'
 * 输出: false
 *
 * 注意：
 *
 *
 * A 和 B 长度不超过 100。
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 将a复制一倍为aa；
    /// - 如果aa包含b,则b必定为a的旋转字符串；
    pub fn rotate_string(a: String, b: String) -> bool {
        a.len() == b.len() && a.repeat(2).contains(b.as_str())
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
            Solution::rotate_string("abcde".into(), "cdeab".into()),
            true
        );
        assert_eq!(
            Solution::rotate_string("abcde".into(), "abced".into()),
            false
        );
    }
}
