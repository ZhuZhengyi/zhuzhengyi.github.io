# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/climbing-stairs
@Language: Python
@Datetime: 15-07-26 00:23
'''

class Solution:
    """
    @param n: An integer
    @return: An integer
    """
    def climbStairs(self, n):
        # write your code here
        if n==1:
            return 1
        if n==2:
            return 2
            
        f,s=1,2
        for i in range(3,n+1):
            s,f=f+s,s
            
        return s

