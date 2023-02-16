/*
 * @lc app=leetcode.cn id=224 lang=rust
 *
 * [224] 基本计算器
 *
 * https://leetcode.cn/problems/basic-calculator/description/
 *
 * algorithms
 * Hard (41.92%)
 * Likes:    777
 * Dislikes: 0
 * Total Accepted:    91.5K
 * Total Submissions: 218.2K
 * Testcase Example:  '"1 + 1"'
 *
 * 给你一个字符串表达式 s ，请你实现一个基本计算器来计算并返回它的值。
 * 
 * 注意:不允许使用任何将字符串作为数学表达式计算的内置函数，比如 eval() 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：s = "1 + 1"
 * 输出：2
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：s = " 2-1 + 2 "
 * 输出：3
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：s = "(1+(4+5+2)-3)+(6+8)"
 * 输出：23
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= s.length <= 3 * 10^5
 * s 由数字、'+'、'-'、'('、')'、和 ' ' 组成
 * s 表示一个有效的表达式
 * '+' 不能用作一元运算(例如， "+1" 和 "+(2 + 3)" 无效)
 * '-' 可以用作一元运算(即 "-1" 和 "-(2 + 3)" 是有效的)
 * 输入中不存在两个连续的操作符
 * 每个数字和运行的计算将适合于一个有符号的 32位 整数
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
	/// 对于表达式： 1+(4—(5+2))-3+(6+8)-(10-(19-3))
	///           = 1 + 4 + (-5) + (-2) + (-3) + 6 + 8 + (-10) + (--19) + (---3) 
	/// 可以看出, 结果 = 各个数字与其前面的符号 的 和 组成
	/// 各个数字前面的符号由嵌套括号决定，每多一层括号，
	/// 如果括号前为+, 则括号中的各个数字符号不变；
	/// 如果括号前符号为-, 则括号中各个数字的符号取反；
    pub fn calculate(s: String) -> i32 {
        let bs = s.as_bytes();
        let mut ret: i32 = 0;
        let mut ops = vec![];
        ops.push(1);
        let mut sign: i32 = 1;
        let mut i: usize = 0;
        while i < bs.len() {
            match bs[i] {
                b' ' => {
                    i+=1;
                },
                b'(' => { //
                    ops.push(sign);
                    i+=1;
                },
                b')' => {
                    ops.pop();
                    i+=1;
                },
                b'+' => {
                    sign = ops[ops.len()-1];
                    i+=1;
                },
                b'-' => {
                    sign = -1 * ops[ops.len()-1];
                    i+=1;
                }
                _ => {
                    let mut num = 0;
                    while i<s.len() && matches!(bs[i], b'0'..=b'9')  {
                        num = num*10 + (bs[i] as i32 - b'0' as i32);
                        i+=1;
                    }
                    ret += sign * num;
                }
            }
        }

        ret
    }
}
// @lc code=end

#[cfg(tests)]
mod tests {
    use super::*;

    #[test]
    fn calculate() {

    }

}
