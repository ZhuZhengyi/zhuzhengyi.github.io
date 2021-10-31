# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/find-peak-element
@Language: Python
@Datetime: 15-08-25 14:03
'''

class Solution:
    #@param A: An integers list.
    #@return: return any of peek positions.
    def findPeak(self, A):
        # write your code here
        
        n=len(A)
        
        if n<3:
            return -1
        
        i=n/2
        while i>0 and i<n-1:
            if A[i]>A[i-1] and A[i]>A[i+1]:
                return i
            elif A[i-1]>A[i]:
                i-=1
            else:
                i+=1
    
        return -1
        
        
        
