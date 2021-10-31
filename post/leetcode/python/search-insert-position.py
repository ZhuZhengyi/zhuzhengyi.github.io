# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/search-insert-position
@Language: Python
@Datetime: 15-08-03 08:26
'''

class Solution:
    """
    @param A : a list of integers
    @param target : an integer to be inserted
    @return : an integer
    """
    def searchInsert(self, A, target):
        # write your code here
        n=len(A)
        if n<1:
            return 0
        
        l,r=0,n-1
        while l<=r:
            mid=(l+r)/2
            if target==A[mid]:
                return mid
            elif target<A[mid]:
                r=mid-1
            else:
                l=mid+1
                
        return l
