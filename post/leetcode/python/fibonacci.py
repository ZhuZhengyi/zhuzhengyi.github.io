# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/fibonacci
@Language: Python
@Datetime: 15-07-26 08:41
'''

class Solution:
    # @param n: an integer
    # @return an integer f(n)
    def fibonacci(self, n):
        # write your code here
        if n==1:
            return 0
        if n==2:
            return 1
        
        f,s=0,1
        for i in range(3, n+1):
            s,f=f+s,s
            
        return s

