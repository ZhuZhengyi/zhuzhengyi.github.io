/*
 * @lc app=leetcode.cn id=67 lang=rust
 *
 * [67] 二进制求和
 *
 * https://leetcode-cn.com/problems/add-binary/description/
 *
 * algorithms
 * Easy (54.11%)
 * Likes:    717
 * Dislikes: 0
 * Total Accepted:    210.5K
 * Total Submissions: 389.1K
 * Testcase Example:  '"11"\n"1"'
 *
 * 给你两个二进制字符串，返回它们的和（用二进制表示）。
 * 
 * 输入为 非空 字符串且只包含数字 1 和 0。
 * 
 * 
 * 
 * 示例 1:
 * 
 * 输入: a = "11", b = "1"
 * 输出: "100"
 * 
 * 示例 2:
 * 
 * 输入: a = "1010", b = "1011"
 * 输出: "10101"
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 每个字符串仅由字符 '0' 或 '1' 组成。
 * 1 <= a.length, b.length <= 10^4
 * 字符串如果不是 "0" ，就都不含前导零。
 * 
 * 
 */

// @lc code=start
use std::iter;

impl Solution {
    /// ## 解题思路
    /// 
    pub fn add_binary(a: String, b: String) -> String {
        // 处理长度
        let (a, b) = match (a.len(), b.len()) {
            (la, lb) if la > lb => (a, b),
            _ => (b, a),
        };

        let a = format!( "0{}", a);
        
        //
        let mut ab = a.chars()
            .rev()
            .zip(b.chars().rev().chain(iter::repeat('0')))
            .scan(0_u32, |carry, (x, y)| {
                let s = x as u32 + y as u32 + *carry % 2;
                *carry = s / 2;
                Some(char::from_digit(s%2, 2))
            })
            .map(|c| c.unwrap_or('0'))
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>()
            ;

        if ab.len() > 1 && ab.starts_with('0') {
            String::from(&ab[1..])
        } else {
            ab
        }
    }
}
// @lc code=end

