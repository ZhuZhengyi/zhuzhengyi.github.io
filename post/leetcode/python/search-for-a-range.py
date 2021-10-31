# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/search-for-a-range
@Language: Python
@Datetime: 15-08-25 13:25
'''

class Solution:
    """
    @param A : a list of integers
    @param target : an integer to be searched
    @return : a list of length 2, [index1, index2]
    """
    def searchRange(self, A, target):
        # write your code here
        n=len(A)
        
        if n==1:
            if A[0]==target:
                return [0,0]
            else:
                return [-1,-1]
        
        res=[]
        
        l,r=0,n-1
        while l<r:
            mid=(l+r+1)/2
            if target<A[mid]:
                r=mid-1
            elif target>A[mid]:
                l=mid
            else:
                l,r=mid,mid
                while l>=0 and A[l]==target:
                    l-=1
                while r<n and A[r]==target:
                    r+=1
                if  l<0 or A[l]!=target:
                    l+=1
                if r>mid:
                    r-=1
                return [l,r]
        
        return [-1,-1]
        
