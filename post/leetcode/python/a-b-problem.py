# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/a-b-problem
@Language: Python
@Datetime: 15-08-03 13:57
'''

class Solution:
    """
    @param a: The first integer
    @param b: The second integer
    @return:  The sum of a and b
    """
    def aplusb(self, a, b):
        # write your code here, try to do it without arithmetic operators.
        if b==0:
            return a
        if a==-b:
            return 0
            
        while b!=0:
            a,b=a^b, (a&b)<<1
            
        return a