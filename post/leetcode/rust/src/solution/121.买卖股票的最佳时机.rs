/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 *
 * https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/description/
 *
 * algorithms
 * Easy (52.41%)
 * Likes:    779
 * Dislikes: 0
 * Total Accepted:    137.3K
 * Total Submissions: 260.7K
 * Testcase Example:  '[7,1,5,3,6,4]'
 *
 * 给定一个数组，它的第 i 个元素是一支给定股票第 i 天的价格。
 *
 * 如果你最多只允许完成一笔交易（即买入和卖出一支股票），设计一个算法来计算你所能获取的最大利润。
 *
 * 注意你不能在买入股票前卖出股票。
 *
 * 示例 1:
 *
 * 输入: [7,1,5,3,6,4]
 * 输出: 5
 * 解释: 在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
 * ⁠    注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格。
 *
 *
 * 示例 2:
 *
 * 输入: [7,6,4,3,1]
 * 输出: 0
 * 解释: 在这种情况下, 没有交易完成, 所以最大利润为 0。
 *
 *
 */

use super::*;

// @lc code=start

impl Solution {
    /// ## 解题思路：
    /// - 动态规划+贪心
    /// 1. 设 profit[i]: 表示第i天不同状态下最后得到的最大收益;
    /// 2. 第i天策略最终最大收益如下:
    ///    - 未买入, 此时有如下操作:
    ///      - 不操作. 当天收益: 0, 后续收益: profit[i+1].未买入;
    ///      - 买入. 当天收益: -prices[i], 后续收益: profit[i+1].已买入;
    ///      此时最大总收益为: profit[i].未买入 = max(profit[i+1].未买入, -prices[i]+profit[i+1].已买入)
    ///    - 已买入, 此时操作收益如下:
    ///      - 不操作. 当天收益: 0, 后续收益: profit[i+1].已买入
    ///      - 卖出. 当天收益: prices[i], 后续无法继续买卖, 收益为0;
    ///      此时最大总收益为: profit[i].已买入 = max(profit[i+1].已买入, prices[i]+0)
    ///  3. 最终结果: profit[0].未买入, 表示最开始未买入时到最后
    ///     初始条件: profit[n] = (0, 0)
    ///  4. 优化:
    ///     - profit[i] 只和 price[i], profit[i+1]有关, 可使用一个变量来记录profit;
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .into_iter()
            .fold((0, 0), |profit, p| {
                (profit.0.max(-p + profit.1), profit.1.max(p + 0))
            })
            .0
    }

    /// ## 解题思路2：
    /// [0, i]天的最大收益f(i):
    ///     f(i) = max(f(i-1), prices(i) - min(prcices[:i]))
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0 as i32;
        }
        let mut min_price = prices[0];
        let mut max_profit = 0;
        for i in 1..prices.len() {
            min_price = std::cmp::min(min_price, prices[i]);
            max_profit = std::cmp::max(max_profit, prices[i] - min_price);
        }

        max_profit
    }
}
// @lc code=end
