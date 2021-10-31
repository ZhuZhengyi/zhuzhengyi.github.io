# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/longest-increasing-continuous-subsequence
@Language: Python
@Datetime: 15-08-02 23:56
'''

class Solution:
    # @param {int[]} A an array of Integer
    # @return {int}  an integer
    def longestIncreasingContinuousSubsequence(self, A):
        # Write your code here
        n=len(A)
        if n<=1:
            return n
        l_up=0
        l_down=0
        
        d=-1   #0,up; 1,down; -1: init
        l_cur=1
        for i in range(1,n):
            if A[i]>A[i-1]:
                if d!=1:
                    l_cur+=1
                else:
                    l_cur=2
                d=0
                l_up=max(l_up, l_cur)
            else:
                if d!=0:
                    l_cur+=1
                else:
                    l_cur=2
                d=1
                l_down=max(l_down, l_cur)
        
        return max(l_up, l_down)
        
        
        
        
        
