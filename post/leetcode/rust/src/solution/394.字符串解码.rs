/*
 * @lc app=leetcode.cn id=394 lang=rust
 *
 * [394] 字符串解码
 *
 * https://leetcode.cn/problems/decode-string/description/
 *
 * algorithms
 * Medium (56.90%)
 * Likes:    1423
 * Dislikes: 0
 * Total Accepted:    225.2K
 * Total Submissions: 396.8K
 * Testcase Example:  '"3[a]2[bc]"'
 *
 * 给定一个经过编码的字符串，返回它解码后的字符串。
 *
 * 编码规则为: k[encoded_string]，表示其中方括号内部的 encoded_string 正好重复 k 次。注意 k 保证为正整数。
 *
 * 你可以认为输入字符串总是有效的；输入字符串中没有额外的空格，且输入的方括号总是符合格式要求的。
 *
 * 此外，你可以认为原始数据不包含数字，所有的数字只表示重复的次数 k ，例如不会出现像 3a 或 2[4] 的输入。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "3[a]2[bc]"
 * 输出："aaabcbc"
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = "3[a2[c]]"
 * 输出："accaccacc"
 *
 *
 * 示例 3：
 *
 *
 * 输入：s = "2[abc]3[cd]ef"
 * 输出："abcabccdcdcdef"
 *
 *
 * 示例 4：
 *
 *
 * 输入：s = "abc3[cd]xyz"
 * 输出："abccdcdcdxyz"
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= s.length <= 30
 * s 由小写英文字母、数字和方括号 '[]' 组成
 * s 保证是一个 有效 的输入。
 * s 中所有整数的取值范围为 [1, 300]
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 栈
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<(String, usize)> = Vec::new(); // 栈, 保存遍历时已经解码的字符串和当前`[]`区间的times
        let mut decoded_str = String::new(); // 已解码的字符串
        let mut times = 0; // 区间次数
        for c in s.chars() {
            match c {
                '[' => {
                    //进入[]区间，
                    stack.push((decoded_str.clone(), times)); //将之前已解码字符串和当前区间的times入栈

                    times = 0; //重置区间计数
                    decoded_str.clear(); //重置已解码字符串
                }
                ']' => {
                    // 离开区间,
                    if let Some((pre_decoded_str, this_times)) = stack.pop() {
                        // 将当前区间已解码字符串重复times，并附加到该区间前已解码字符串后，
                        // 得到完整已解码字符串
                        decoded_str = pre_decoded_str + decoded_str.repeat(this_times).as_str();
                    }
                }
                '0'..='9' => times = 10 * times + (c as u8 - b'0') as usize, //计算times
                _ => {
                    decoded_str.push(c);
                }
            }
        }

        decoded_str
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
            Solution::decode_string("3[a]2[bc]".into()),
            "aaabcbc".to_string()
        );
        assert_eq!(
            Solution::decode_string("3[a2[c]]".into()),
            "accaccacc".to_string()
        );
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".into()),
            "abcabccdcdcdef".to_string()
        );
        assert_eq!(
            Solution::decode_string("abc3[cd]xyz".into()),
            "abccdcdcdxyz".to_string()
        );
        assert_eq!(
            Solution::decode_string("abc2[mno11[f]xy]".into()),
            "abcmnofffffffffffxymnofffffffffffxy".to_string()
        );
    }
}
