/*
 * @lc app=leetcode.cn id=29 lang=rust
 *
 * [29] 两数相除
 *
 * https://leetcode-cn.com/problems/divide-two-integers/description/
 *
 * algorithms
 * Medium (20.19%)
 * Likes:    465
 * Dislikes: 0
 * Total Accepted:    71.1K
 * Total Submissions: 351.7K
 * Testcase Example:  '10\n3'
 *
 * 给定两个整数，被除数 dividend 和除数 divisor。将两数相除，要求不使用乘法、除法和 mod 运算符。
 * 
 * 返回被除数 dividend 除以除数 divisor 得到的商。
 * 
 * 整数除法的结果应当截去（truncate）其小数部分，例如：truncate(8.345) = 8 以及 truncate(-2.7335) =
 * -2
 * 
 * 
 * 
 * 示例 1:
 * 
 * 输入: dividend = 10, divisor = 3
 * 输出: 3
 * 解释: 10/3 = truncate(3.33333..) = truncate(3) = 3
 * 
 * 示例 2:
 * 
 * 输入: dividend = 7, divisor = -3
 * 输出: -2
 * 解释: 7/-3 = truncate(-2.33333..) = -2
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 被除数和除数均为 32 位有符号整数。
 * 除数不为 0。
 * 假设我们的环境只能存储 32 位有符号整数，其数值范围是 [−2^31,  2^31 − 1]。本题中，如果除法结果溢出，则返回 2^31 − 1。
 * 
 * 
 */

struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 1. 先处理符号，被除数、除数都取绝对值；
    /// 2. 试着将除数逐渐翻倍，直到比被除数大为止，记翻倍次数为n，此时结果应该在(n/2, n)之间;
    /// 3. 从n/2开始，依次
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let negative = (dividend < 0) != (divisor < 0);
        let (mut dividend, mut divisor) = (i64::from(dividend).abs(), i64::from(divisor).abs());
        let mut n = 1;
        let mut res = 0_i64;
        //将divisor依次左移，直到大于diviend（快增长）
        while (divisor << 1) <= dividend {
            divisor <<= 1;      //除数
            n <<= 1;            //记录倍数
        }
        //(慢试探)
        while n > 0 {
            if dividend >= divisor {
                res += n;
                dividend -= divisor;
            }
            divisor >>= 1;
            n >>= 1;
        }

        if negative {
            -res as i32
        } else {
            res.min(i64::from(i32::MAX)) as i32
        }
    }
}
// @lc code=end

