# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/reverse-integer
@Language: Python
@Datetime: 15-07-27 00:49
'''

class Solution:
    # @param {int} n the integer to be reversed
    # @return {int} the reversed integer
    def reverseInteger(self, n):
        # Write your code here
        if n==0:
            return 0
        
        if n<0 :
            return -self.reverseInteger(-n)
            
        maxint32=2147483647
        s=0
        while n>0:
            t=n%10
            if s>=(maxint32-t)/10:
                return 0
            s=s*10+t
            n=n/10
            
            
        return s