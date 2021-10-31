# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/remove-duplicates-from-sorted-array
@Language: Python
@Datetime: 15-08-23 03:47
'''

class Solution:
    """
    @param A: a list of integers
    @return an integer
    """
    def removeDuplicates(self, A):
        # write your code here
        n=len(A)
        
        if n<2:
            return n
        
        j=0
        for i in range(1,n):
            if A[i]!=A[j]:
                j+=1
                A[j]=A[i]
            
        return j+1
        
        
