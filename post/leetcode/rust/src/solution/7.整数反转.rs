/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 *
 * https://leetcode.cn/problems/reverse-integer/description/
 *
 * algorithms
 * Medium (35.38%)
 * Likes:    3657
 * Dislikes: 0
 * Total Accepted:    1.1M
 * Total Submissions: 3.1M
 * Testcase Example:  '123'
 *
 * 给你一个 32 位的有符号整数 x ，返回将 x 中的数字部分反转后的结果。
 * 
 * 如果反转后整数超过 32 位的有符号整数的范围 [−2^31,  2^31 − 1] ，就返回 0。
 * 假设环境不允许存储 64 位整数（有符号或无符号）。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：x = 123
 * 输出：321
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：x = -123
 * 输出：-321
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：x = 120
 * 输出：21
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：x = 0
 * 输出：0
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * -2^31 
 * 
 * 
 */

use super::*;

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x > 1<<31 - 1 || x < -1 * 1<<31 {
            return 0;
        }

        let (mut x1, flag) = match x {
            x if x >=0 => (x,1),
            _ => (-x, -1)
        };

        let mut y1 = 0 ;
        while x1 > 0 {
            let a = x1 % 10; 
            y1 = 10 * y1 + a ;
            x1 /= 10;
        }

        if y1 > 1<<31-1 || y1 < -1 * 1<<31 {
            0
        } else {
            flag * y1
        }

    }
}
// @lc code=end

