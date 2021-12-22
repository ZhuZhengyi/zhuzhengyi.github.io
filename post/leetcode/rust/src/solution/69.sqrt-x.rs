/*
 * @lc app=leetcode.cn id=69 lang=rust
 *
 * [69] Sqrt(x)
 *
 * https://leetcode-cn.com/problems/sqrtx/description/
 *
 * algorithms
 * Easy (39.08%)
 * Likes:    842
 * Dislikes: 0
 * Total Accepted:    425.4K
 * Total Submissions: 1.1M
 * Testcase Example:  '4'
 *
 * 给你一个非负整数 x ，计算并返回 x 的 算术平方根 。
 * 
 * 由于返回类型是整数，结果只保留 整数部分 ，小数部分将被 舍去 。
 * 
 * 注意：不允许使用任何内置指数函数和算符，例如 pow(x, 0.5) 或者 x ** 0.5 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：x = 4
 * 输出：2
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：x = 8
 * 输出：2
 * 解释：8 的算术平方根是 2.82842..., 由于返回类型是整数，小数部分将被舍去。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= x <= 2^31 - 1
 * 
 * 
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// 二分法
    pub fn my_sqrt(x: i32) -> i32 {
        use std::cmp::Ordering;

        if x < 2 {
            return x;
        }
        let mut res = 0;
        let (mut l, mut r) = (0, x/2 + 1);
        while l <= r {
            let m = l + (r - l) / 2;
            match m.cmp(&(x/m)) {
                Ordering::Equal => return m,
                Ordering::Greater => r = m - 1,
                Ordering::Less => {
                    l = m + 1;
                    res = m
                }
            }
        }

        res
    }
}
// @lc code=end

