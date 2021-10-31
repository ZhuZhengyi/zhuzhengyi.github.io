# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/plus-one
@Language: Python
@Datetime: 15-07-27 00:49
'''

class Solution:
    # @param {int[]} digits a number represented as an array of digits
    # @return {int[]} the result
    def plusOne(self, digits):
        # Write your code here
        c=1
        n=len(digits)
        res=[]
        for d in range(n-1,-1,-1):
            s=digits[d]+c
            c=1 if s>9 else 0
            res.insert(0, s%10)
        
        if c==1:
            res.insert(0,1)
        
        return res
            
            