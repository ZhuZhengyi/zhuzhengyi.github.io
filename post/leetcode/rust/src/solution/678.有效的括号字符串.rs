/*
 * @lc app=leetcode.cn id=678 lang=rust
 *
 * [678] 有效的括号字符串
 *
 * https://leetcode.cn/problems/valid-parenthesis-string/description/
 *
 * algorithms
 * Medium (39.23%)
 * Likes:    553
 * Dislikes: 0
 * Total Accepted:    64.7K
 * Total Submissions: 165K
 * Testcase Example:  '"()"'
 *
 * 给定一个只包含三种字符的字符串：（ ，） 和 *，写一个函数来检验这个字符串是否为有效字符串。有效字符串具有如下规则：
 *
 *
 * 任何左括号 ( 必须有相应的右括号 )。
 * 任何右括号 ) 必须有相应的左括号 ( 。
 * 左括号 ( 必须在对应的右括号之前 )。
 * * 可以被视为单个右括号 ) ，或单个左括号 ( ，或一个空字符串。
 * 一个空字符串也被视为有效字符串。
 *
 *
 * 示例 1:
 *
 *
 * 输入: "()"
 * 输出: True
 *
 *
 * 示例 2:
 *
 *
 * 输入: "(*)"
 * 输出: True
 *
 *
 * 示例 3:
 *
 *
 * 输入: "(*))"
 * 输出: True
 *
 *
 * 注意:
 *
 *
 * 字符串大小将在 [1，100] 范围内。
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 贪心法
    /// 1. 从左到右遍历每个字符；
    /// 2. 设置2个变量，分别
    ///     - unpaired_lefts: 当前未匹配的'('数；
    ///     - can_pair_rights: 可匹配')'的数量；
    /// 3. 对于不同字符，分别处理；
    pub fn check_valid_string(s: String) -> bool {
        let mut unpaired_lefts = 0; //未匹配的'('数
        let mut can_pair_rights = 0; //可匹配')'的容量
        for c in s.chars() {
            match c {
                '(' => {
                    can_pair_rights += 1; // 可匹配')'的数+1
                    unpaired_lefts += 1; // 未匹配的'('数+1
                }
                ')' => {
                    // 如果可匹配')'的容量为空, 则匹配失败
                    if can_pair_rights <= 0 {
                        return false;
                    }

                    can_pair_rights -= 1; // 否则,可匹配')'的容量-1

                    // 如果存在未匹配的'('
                    if unpaired_lefts > 0 {
                        unpaired_lefts -= 1; //则将未匹配的'('-1
                    }
                }
                '*' => {
                    can_pair_rights += 1; // 可匹配')'的容量+1

                    // 如果有未匹配的'('
                    if unpaired_lefts > 0 {
                        unpaired_lefts -= 1; // 未匹配的'('计数-1
                    };
                }
                _ => {}
            }
        }

        unpaired_lefts == 0 //是否剩下未匹配的'('
    }
}
// @lc code=end

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(Solution::check_valid_string("()".into()), true);
        assert_eq!(Solution::check_valid_string("(*)".into()), true);
        assert_eq!(Solution::check_valid_string("())".into()), false);
        assert_eq!(Solution::check_valid_string("*)".into()), true);
        assert_eq!(Solution::check_valid_string("(*".into()), true);
        assert_eq!(Solution::check_valid_string("(*))".into()), true);
    }
}
