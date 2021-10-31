# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/sort-letters-by-case
@Language: Python
@Datetime: 15-08-25 12:42
'''

class Solution:
    """
    @param chars: The letters array you should sort.
    """
    def sortLetters(self, chars):
        # write your code here
        n=len(chars)
        
        if n<=1:
            return 
        
        i,j=0,n-1
        while i<j:
            while i<j and chars[i].islower():
                i+=1
            while j>i and chars[j].isupper():
                j-=1
            if i<j:
                chars[i],chars[j]=chars[j],chars[i]

