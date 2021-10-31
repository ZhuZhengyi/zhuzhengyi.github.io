# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/best-time-to-buy-and-sell-stock
@Language: Python
@Datetime: 15-07-26 00:15
'''

class Solution:
    """
    @param prices: Given an integer array
    @return: Maximum profit
    """
    def maxProfit(self, prices):
        # write your code here
        if len(prices)<1:
            return 0
            
        max_profit=0
        min_price=prices[0]
        for p in prices:
            max_profit=max(max_profit, p-min_price)
            min_price=min(min_price, p)
            
        return max_profit
