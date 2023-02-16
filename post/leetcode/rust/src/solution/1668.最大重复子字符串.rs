/*
 * @lc app=leetcode.cn id=1668 lang=rust
 *
 * [1668] 最大重复子字符串
 *
 * https://leetcode-cn.com/problems/maximum-repeating-substring/description/
 *
 * algorithms
 * Easy (44.41%)
 * Likes:    18
 * Dislikes: 0
 * Total Accepted:    7.5K
 * Total Submissions: 16.9K
 * Testcase Example:  '"ababc"\n"ab"'
 *
 * 给你一个字符串 sequence ，如果字符串 word 连续重复 k 次形成的字符串是 sequence 的一个子字符串，那么单词 word 的
 * 重复值为 k 。单词 word 的 最大重复值 是单词 word 在 sequence 中最大的重复值。如果 word 不是 sequence
 * 的子串，那么重复值 k 为 0 。
 * 
 * 给你一个字符串 sequence 和 word ，请你返回 最大重复值 k 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：sequence = "ababc", word = "ab"
 * 输出：2
 * 解释："abab" 是 "ababc" 的子字符串。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：sequence = "ababc", word = "ba"
 * 输出：1
 * 解释："ba" 是 "ababc" 的子字符串，但 "baba" 不是 "ababc" 的子字符串。
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：sequence = "ababc", word = "ac"
 * 输出：0
 * 解释："ac" 不是 "ababc" 的子字符串。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 1 
 * sequence 和 word 都只包含小写英文字母。
 * 
 * 
 */

use super::*;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let max_word = word.repeat(sequence.len()/word.len());
        let mut l = max_word.len();
        while !sequence.contains(&max_word[..l]) {
            l -= word.len();
        }

        (l / word.len()) as _
    }
}
// @lc code=end

