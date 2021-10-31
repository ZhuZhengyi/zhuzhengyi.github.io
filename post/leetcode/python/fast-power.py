# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/fast-power
@Language: Python
@Datetime: 15-08-06 13:04
'''

class Solution:
    """
    @param a, b, n: 32bit integers
    @return: An integer
    """
    def fastPower(self, a, b, n):
        # write your code here
        if n==0:
            return 1%b
        
        if n==1:
            return a%b
        
        if a==1:
            return 1%b
            
        res=self.fastPower(a,b,n/2)%b
        if n%2!=0:
            res=(res*res*(a%b))%b
        else:
            res=(res*res)%b
        
        return res