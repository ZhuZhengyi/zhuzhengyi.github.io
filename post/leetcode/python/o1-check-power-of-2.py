# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/o1-check-power-of-2
@Language: Python
@Datetime: 15-08-30 03:31
'''

class Solution:
    """
    @param n: An integer
    @return: True or false
    """
    def checkPowerOf2(self, n):
        # write your code here
        if n==0:
            return False
        return n&(n-1) == 0 
        