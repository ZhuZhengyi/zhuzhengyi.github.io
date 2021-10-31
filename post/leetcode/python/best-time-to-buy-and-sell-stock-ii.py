# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/best-time-to-buy-and-sell-stock-ii
@Language: Python
@Datetime: 15-08-23 07:46
'''

class Solution:
    """
    @param prices: Given an integer array
    @return: Maximum profit
    """
    def maxProfit(self, prices):
        # write your code here
        n=len(prices)
        s=0
        r=0
        for i in range(1,n):
            if prices[i]>prices[i-1]:
                s=prices[i]-prices[i-1]
            else:
                s=0
            r+=s
        
        return r