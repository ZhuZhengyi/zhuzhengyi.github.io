/*
 * @lc app=leetcode.cn id=322 lang=rust
 *
 * [322] 零钱兑换
 *
 * https://leetcode-cn.com/problems/coin-change/description/
 *
 * algorithms
 * Medium (44.54%)
 * Likes:    1549
 * Dislikes: 0
 * Total Accepted:    319.8K
 * Total Submissions: 716.9K
 * Testcase Example:  '[1,2,5]\n11'
 *
 * 给你一个整数数组 coins ，表示不同面额的硬币；以及一个整数 amount ，表示总金额。
 * 
 * 计算并返回可以凑成总金额所需的 最少的硬币个数 。如果没有任何一种硬币组合能组成总金额，返回 -1 。
 * 
 * 你可以认为每种硬币的数量是无限的。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：coins = [1, 2, 5], amount = 11
 * 输出：3 
 * 解释：11 = 5 + 5 + 1
 * 
 * 示例 2：
 * 
 * 
 * 输入：coins = [2], amount = 3
 * 输出：-1
 * 
 * 示例 3：
 * 
 * 
 * 输入：coins = [1], amount = 0
 * 输出：0
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：coins = [1], amount = 1
 * 输出：1
 * 
 * 
 * 示例 5：
 * 
 * 
 * 输入：coins = [1], amount = 2
 * 输出：2
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 1 
 * 0 
 * 
 * 
 */

// @lc code=start
use std::cmp;

impl Solution {
    /// 解题思路：
    /// 对于总量为n的金额，最少所需硬币个数f(n):
    ///   f(n) = min(f(n-coins[i])) + 1
    /// 为此需要依次计算出f(i)的值
    /// 初始条件:
    ///   f(0) = 0
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut res = vec![None; amount+1];
        res[0] = Some(0);

        for i in 1..=amount {
            res[i] = coins.iter()
                        .filter_map(|&c| {
                            let c = c as usize;
                            if c <= i {
                                res[i-c].map(|n| n + 1)
                            } else {
                                None
                            }
                        })
                        .min();
        }

        res[amount].unwrap_or(-1)
    }
}
// @lc code=end

