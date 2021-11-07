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

// @lc code=start
use std::cmp;

impl Solution {
    /// 解题思路：
    /// [0, i]天的最大收益f(i): 
    ///     f(i) = max(f(i-1), prices(i) - min(prcices[:i]))
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0 as i32;
        }
        let mut min_price = prices[0];
        let mut max_profit = 0;
        for i in 1..prices.len() {
            min_price = cmp::min(min_price, prices[i]);
            max_profit = cmp::max(max_profit, prices[i] - min_price);
        }

        max_profit
    }
}
// @lc code=end

