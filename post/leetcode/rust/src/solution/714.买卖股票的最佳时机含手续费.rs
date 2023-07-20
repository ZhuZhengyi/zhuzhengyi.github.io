/*
 * @lc app=leetcode.cn id=714 lang=rust
 *
 * [714] 买卖股票的最佳时机含手续费
 *
 * https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/description/
 *
 * algorithms
 * Medium (75.23%)
 * Likes:    928
 * Dislikes: 0
 * Total Accepted:    220K
 * Total Submissions: 292.3K
 * Testcase Example:  '[1,3,2,8,4,9]\n2'
 *
 * 给定一个整数数组 prices，其中 prices[i]表示第 i 天的股票价格 ；整数 fee 代表了交易股票的手续费用。
 *
 * 你可以无限次地完成交易，但是你每笔交易都需要付手续费。如果你已经购买了一个股票，在卖出它之前你就不能再继续购买股票了。
 *
 * 返回获得利润的最大值。
 *
 * 注意：这里的一笔交易指买入持有并卖出股票的整个过程，每笔交易你只需要为支付一次手续费。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：prices = [1, 3, 2, 8, 4, 9], fee = 2
 * 输出：8
 * 解释：能够达到的最大利润:
 * 在此处买入 prices[0] = 1
 * 在此处卖出 prices[3] = 8
 * 在此处买入 prices[4] = 4
 * 在此处卖出 prices[5] = 9
 * 总利润: ((8 - 1) - 2) + ((9 - 4) - 2) = 8
 *
 * 示例 2：
 *
 *
 * 输入：prices = [1,3,7,5,10,3], fee = 3
 * 输出：6
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= prices.length <= 5 * 10^4
 * 1 <= prices[i] < 5 * 10^4
 * 0 <= fee < 5 * 10^4
 *
 *
 */


// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划+贪心
    /// 1. 此题在122.买卖股票最佳时机(不限交易次数)基础上, 为每笔交易增加了交易费用fee
    ///    由于每笔股票交易只需要支付一次手续费,
    ///    所以在每次买入时, 将交易费用fee从收益中扣除即可;
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        prices
            .iter()
            .rev()
            .fold((0, 0), |profit, &p| {
                (
                    profit.0.max(-p + profit.1 - fee),
                    profit.1.max(p + profit.0),
                )
            })
            .0
    }
}
// @lc code=end

struct Solution;