/*
 * @lc app=leetcode.cn id=878 lang=rust
 *
 * [878] 第 N 个神奇数字
 *
 * https://leetcode-cn.com/problems/nth-magical-number/description/
 *
 * algorithms
 * Hard (28.82%)
 * Likes:    97
 * Dislikes: 0
 * Total Accepted:    5K
 * Total Submissions: 17.3K
 * Testcase Example:  '1\n2\n3'
 *
 * 如果正整数可以被 A 或 B 整除，那么它是神奇的。
 *
 * 返回第 N 个神奇数字。由于答案可能非常大，返回它模 10^9 + 7 的结果。
 *
 *
 *
 *
 *
 *
 * 示例 1：
 *
 * 输入：N = 1, A = 2, B = 3
 * 输出：2
 *
 *
 * 示例 2：
 *
 * 输入：N = 4, A = 2, B = 3
 * 输出：6
 *
 *
 * 示例 3：
 *
 * 输入：N = 5, A = 2, B = 4
 * 输出：10
 *
 *
 * 示例 4：
 *
 * 输入：N = 3, A = 6, B = 4
 * 输出：8
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= N <= 10^9
 * 2 <= A <= 40000
 * 2 <= B <= 40000
 *
 *
 */

struct Solution;

// @lc code=start
impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut n = n;
        let mut res: i64 = 0;
        for i in 2..MOD {
            if (i as i32) % a == 0 || (i as i32) % b == 0 {
                n -= 1;
            }
            if n == 0 {
                res = i;
                break;
            }
        }

        //res as i32
        (res % MOD) as i32
    }
}

// @lc code=end

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::nth_magical_number(4, 2, 3), 6);
    }
}
