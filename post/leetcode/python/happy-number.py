# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/happy-number
@Language: Python
@Datetime: 15-12-17 07:55
'''

class Solution:
    # @param {int} n an integer
    # @return {boolean} true if this is a happy number or false
    def isHappy(self, n):
        # Write your code here
        
        Hash={}
        while n!=1:
            if n in Hash: 
                return False
            else: 
                Hash[n] = True
            
            n=sum([int(c)*int(c)  for c in str(n)])
        
        return True
        