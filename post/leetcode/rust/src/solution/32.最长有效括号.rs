/*
 * @lc app=leetcode.cn id=32 lang=rust
 *
 * [32] 最长有效括号
 *
 * https://leetcode-cn.com/problems/longest-valid-parentheses/description/
 *
 * algorithms
 * Hard (29.67%)
 * Likes:    543
 * Dislikes: 0
 * Total Accepted:    44.1K
 * Total Submissions: 147.6K
 * Testcase Example:  '"(()"'
 *
 * 给定一个只包含 '(' 和 ')' 的字符串，找出最长的包含有效括号的子串的长度。
 * 
 * 示例 1:
 * 
 * 输入: "(()"
 * 输出: 2
 * 解释: 最长有效括号子串为 "()"
 * 
 * 
 * 示例 2:
 * 
 * 输入: ")()())"
 * 输出: 4
 * 解释: 最长有效括号子串为 "()()"
 * 
 * 
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 设 f[i]: 以s[i]为尾的最长有效括号长度, 题目转化为 max(f[i]) (i=0..n-1)。
    /// 则 f[i-1]: 以s[i-1]为尾的最长有效括号长度，对于f[i], 有如下关系: 
    /// 1. 如果s[i]是'('，则以其为结尾不是有效括号，因此 f[i] = 0;            
    /// 2. 如果s[i]是')'，以s[i-1]为尾的最长有效括号字符串为s[i-f[i-1]:i]。 
    ///   示意图：   
    ///         pre_index
    ///             |
    ///             v| f[i-1] |
    ///         *****(********))
    ///              ^        ^
    ///           i-f[i-1]  (i-1)     
    ///   其前一个字符为s[i-f[i-1]-1], 记 pre_index=i-f[i-1]-1
    ///   - 如果s[pre_index]为'(', 则s[pre_index:i+1]也为有效括号串, 即:
    ///           f[i] = f[i-1] + 2     
    ///   示意图：  ******((********))
    ///                  ^          ^
    ///             pre_index       i
    ///   - 此时，如果pre_index前仍然存在字符串，则最长有效长度还加上前面部分的有效长度，
    ///   使前后连贯起来, 因此：
    ///             f[i] += f[pre_index-1]
    ///                      f[i-1] 
    ///   示意图           |<------->|i
    ///          ***(***)(**********)
    ///   f[pre_index-1] ^           
    ///               pre_index   
    pub fn longest_valid_parentheses(s: String) -> i32 {
        // 长度<2的不存在
        if s.len() < 2 {
            return 0;
        }
        let mut f = vec![0; s.len()];   //f[i]: 以s[i]结尾的最长有效括号长度
        for i in 1..s.len() {
            if s.chars().nth(i) == Some( ')' ) {
                let pre_index = i as i32 - f[i-1] - 1; //跳过以s[i-1]为结尾大最长有效括号
                if pre_index >=0 && s.chars().nth(pre_index as usize) == Some('(') {
                    f[i] = f[i-1] + 2;
                    if pre_index > 0 {
                        f[i] += f[(pre_index as usize)-1];
                    }
                }
            }
        }

        *f.iter().max().unwrap()
    }
}
// @lc code=end

