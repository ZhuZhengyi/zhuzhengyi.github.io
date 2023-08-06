/*
 * @lc app=leetcode.cn id=50 lang=rust
 *
 * [50] Pow(x, n)
 *
 * https://leetcode.cn/problems/powx-n/description/
 *
 * algorithms
 * Medium (38.01%)
 * Likes:    1167
 * Dislikes: 0
 * Total Accepted:    370.3K
 * Total Submissions: 974.2K
 * Testcase Example:  '2.00000\n10'
 *
 * 实现 pow(x, n) ，即计算 x 的整数 n 次幂函数（即，x^n^ ）。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：x = 2.00000, n = 10
 * 输出：1024.00000
 *
 *
 * 示例 2：
 *
 *
 * 输入：x = 2.10000, n = 3
 * 输出：9.26100
 *
 *
 * 示例 3：
 *
 *
 * 输入：x = 2.00000, n = -2
 * 输出：0.25000
 * 解释：2^-2 = 1/2^2 = 1/4 = 0.25
 *
 *
 *
 *
 * 提示：
 *
 *
 * -100.0 < x < 100.0
 * -2^31 <= n <= 2^31-1
 * n 是一个整数
 * -10^4 <= x^n <= 10^4
 *
 *
 */

pub struct Solution;

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 二分+递归
    pub fn my_pow(x: f64, n: i32) -> f64 {
        match (x, n) {
            (1_f64, _) => 1_f64,
            (_, 0) => 1_f64,
            (_, 1) => x,
            (-1_f64, n) if n % 2 == 0 => 1_f64,
            (-1_f64, n) if n % 2 != 0 => -1_f64,
            (_, -2147483648) => 0_f64,
            (_, n) if n < 0 => 1_f64 / Solution::my_pow(x, -n),
            (_, n) if n % 2 == 0 => Solution::my_pow(x, n / 2) * Solution::my_pow(x, n / 2),
            _ => Solution::my_pow(x, n - 1) * x,
        }
    }
}
// @lc code=end
