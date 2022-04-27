/*
 * @lc app=leetcode.cn id=121 lang=cpp
 *
 * [121] 买卖股票的最佳时机
 *
 * https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/description/
 *
 * algorithms
 * Easy (57.70%)
 * Likes:    2311
 * Dislikes: 0
 * Total Accepted:    769.8K
 * Total Submissions: 1.3M
 * Testcase Example:  '[7,1,5,3,6,4]'
 *
 * 给定一个数组 prices ，它的第 i 个元素 prices[i] 表示一支给定股票第 i 天的价格。
 * 
 * 你只能选择 某一天 买入这只股票，并选择在 未来的某一个不同的日子 卖出该股票。设计一个算法来计算你所能获取的最大利润。
 * 
 * 返回你可以从这笔交易中获取的最大利润。如果你不能获取任何利润，返回 0 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：[7,1,5,3,6,4]
 * 输出：5
 * 解释：在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
 * ⁠    注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格；同时，你不能在买入前卖出股票。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：prices = [7,6,4,3,1]
 * 输出：0
 * 解释：在这种情况下, 没有交易完成, 所以最大利润为 0。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 0 
 * 
 * 
 */

#include <vector>

using namespace std;

// @lc code=start
class Solution {
public:
    /**
     * ## 解题思路
     * * 贪心法
     * * 由于股票只能先买后卖
    */
    int maxProfit(vector<int>& prices) {
        int maxProfile = 0;
        int minHistoryPrice = 10000;
        for(int p: prices) {
            // 如果当前价格低于历史最低价格，
            if (p<minHistoryPrice) {
                minHistoryPrice = p; //则更新历史最低价格
            }
            // 如果当前最高利润 > 最高利润
            if (p-minHistoryPrice>maxProfile) {
                maxProfile = p-minHistoryPrice;  //更新最高利润
            }
        }

        return maxProfile;
    }
};
// @lc code=end

