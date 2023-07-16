/*
 * @lc app=leetcode.cn id=309 lang=rust
 *
 * [309] 最佳买卖股票时机含冷冻期
 *
 * https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-with-cooldown/description/
 *
 * algorithms
 * Medium (64.14%)
 * Likes:    1539
 * Dislikes: 0
 * Total Accepted:    263.1K
 * Total Submissions: 410K
 * Testcase Example:  '[1,2,3,0,2]'
 *
 * 给定一个整数数组prices，其中第  prices[i] 表示第 i 天的股票价格 。​
 *
 * 设计一个算法计算出最大利润。在满足以下约束条件下，你可以尽可能地完成更多的交易（多次买卖一支股票）:
 *
 *
 * 卖出股票后，你无法在第二天买入股票 (即冷冻期为 1 天)。
 *
 *
 * 注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
 *
 *
 *
 * 示例 1:
 *
 *
 * 输入: prices = [1,2,3,0,2]
 * 输出: 3
 * 解释: 对应的交易状态为: [买入, 卖出, 冷冻期, 买入, 卖出]
 *
 * 示例 2:
 *
 *
 * 输入: prices = [1]
 * 输出: 0
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= prices.length <= 5000
 * 0 <= prices[i] <= 1000
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 动态规划
    /// 1. 设 profit[i]: 表示第i天
    /// 2. 投资者在整个过程中的每天存在3种状态:
    ///    - 未投资(uninvested): 此时投资者有两种策略:
    ///       a1. 不操作. 当天收益为: 0, 后续收益为: profit[i+1].未投资;
    ///       a2. 买入, 当天收益为: -prices[i]. 后续收益为: profit[i+1].已投资;
    ///      此时最终最大收益为:
    ///        profit[i].未投资 = max(0 + profit[i+1].未投资, -prices[i] + profit[i+1].已投资);
    ///    - 已投资(invested): 此时投资者也有两种策略:
    ///       b1. 不操作. 当天收益为: 0, 后期收益为: profit[i+1].已投资;
    ///       b2. 卖出. 当天收益为: +prices[i]. 后期收益为: profit[i+1].冷静期;
    ///      此时最终最大收益为:
    ///        profit[i].已投资 = max(0+profit[i+1].已投资, prices[i]+profit[i+1].冷静期)
    ///    - 冷静期(cooldown): 此时投资者处于冷静期,此时只有一个策略:
    ///       - 无法买卖. 当天收益为: 0. 后期收益为: profit[i+1].未投资;
    ///      此时最终最大收益为:
    ///        profit[i].冷静期 = 0 + profit[i+1].未投资
    /// * 优化:
    ///    - profit[i] 只和profit[i+1], prices[i] 相关, 使用滚动变量代替数组;
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .rev()
            .fold((0, 0, 0), |profit, &p| {
                (
                    profit.0.max(-p + profit.1),
                    profit.1.max(p + profit.2),
                    profit.0,
                )
            })
            .0
    }
}
// @lc code=end

struct Solution;
