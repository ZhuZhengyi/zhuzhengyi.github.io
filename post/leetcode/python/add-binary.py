# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/add-binary
@Language: Python
@Datetime: 15-07-29 13:48
'''

class Solution:
    # @param {string} a a number
    # @param {string} b a number
    # @return {string} the result
    def addBinary(self, a, b):
        # Write your code here
        res=""
        m,n=len(a), len(b)
        c=0
        i=0
        z=min(m,n)
        while i<z:
            x=int(a[-i-1])+int(b[-i-1])+c
            res=str(x%2)+res
            c = 1 if x>1 else 0
            i+=1
        while i<m:
            x=int(a[-i-1])+c
            res=str(x%2)+res
            c = 1 if x>1 else 0
            i+=1
        while i<n:
            x=int(b[-i-1])+c
            res=str(x%2)+res
            c = 1 if x>1 else 0
            i+=1
        if c>0:
            res=str(c)+res
            
        return res
            
        
        
