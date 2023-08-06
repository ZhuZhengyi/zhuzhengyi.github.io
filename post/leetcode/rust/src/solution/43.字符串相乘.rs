/*
 * @lc app=leetcode.cn id=43 lang=rust
 *
 * [43] 字符串相乘
 *
 * https://leetcode-cn.com/problems/multiply-strings/description/
 *
 * algorithms
 * Medium (44.98%)
 * Likes:    838
 * Dislikes: 0
 * Total Accepted:    199K
 * Total Submissions: 442.5K
 * Testcase Example:  '"2"\n"3"'
 *
 * 给定两个以字符串形式表示的非负整数 num1 和 num2，返回 num1 和 num2 的乘积，它们的乘积也表示为字符串形式。
 *
 * 注意：不能使用任何内置的 BigInteger 库或直接将输入转换为整数。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: num1 = "2", num2 = "3"
 * 输出: "6"
 *
 * 示例 2:
 *
 *
 * 输入: num1 = "123", num2 = "456"
 * 输出: "56088"
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= num1.length, num2.length <= 200
 * num1 和 num2 只能由数字组成。
 * num1 和 num2 都不包含任何前导零，除了数字0本身。
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 数学
    /// 1. 2345 * 678 = (2*1000 + 3*100 + 4*10 + 5) * (6*100+7*10+8)
    ///               =   2*1000 * 6*100
    ///                 + 2*1000 * 7*10 + 3*100*6*100
    ///                 + 2*1000 * 8 + 3*100 * 7*10 + 4*10*6*100
    ///                 + 3*100 * 8 + 4*10 * 7*10 + 6*100 * 5
    ///                 + 4*10 * 8 + 5 * 7*10
    ///                 + 5 * 8
    /// 2. 所以 num1[..] * num2[..] = sum(num1[i] * num2[j] * 10^(l1+l2-2)
    pub fn multiply(num1: String, num2: String) -> String {
        match (num1.as_str(), num2.as_str()) {
            (num1, num2) if num1 == "0" || num2 == "0" => "0".to_string(),
            ("1", _) => num2,
            (_, "1") => num1,
            _ => {
                let (num1, num2) = (num1.as_bytes(), num2.as_bytes());
                let (l1, l2) = (num1.len(), num2.len());
                let mut res = String::with_capacity(l1 + l2);
                let mut carry = 0;
                for k in 0..(l1 + l2 - 1) {
                    for i in 0..l1 {
                        let j = k - i;
                        if j < l2 {
                            carry += (num1[l1 - 1 - i] as u32 - b'0' as u32)
                                * (num2[l2 - 1 - j] as u32 - b'0' as u32);
                        }
                    }
                    res.insert(0, char::from_u32(carry % 10 + b'0' as u32).unwrap());
                    carry /= 10;
                }
                if carry > 0 {
                    res.insert(0, char::from_u32(carry + b'0' as u32).unwrap())
                }

                res
            }
        }
    }
}
// @lc code=end
