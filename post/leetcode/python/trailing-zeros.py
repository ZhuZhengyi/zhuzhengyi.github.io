# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/trailing-zeros
@Language: Python
@Datetime: 15-07-31 11:49
'''

class Solution:
    # @param n a integer
    # @return ans a integer
    def trailingZeros(self, n):
        count=0
        while n>0:
            count+=n/5
            n=n/5
            
        return count
        
