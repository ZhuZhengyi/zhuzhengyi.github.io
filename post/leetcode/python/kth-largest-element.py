# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/kth-largest-element
@Language: Python
@Datetime: 15-08-03 00:55
'''

class Solution:
    # @param k & A a integer and an array
    # @return ans a integer
    def kthLargestElement(self, k, A):
        n=len(A)
        if n<1:
            return 0
        if n==1:
            return A[0]
            
        l,r=0, n-1
        while 1:
            l1,r1=l,r
            flag=A[l]
            while l<r:
                while r>l and A[r]<flag:
                    r-=1
                if r>l:
                    A[l]=A[r]
                    l+=1
                while l<r and A[l]>flag:
                    l+=1
                if l<r:
                    A[r]=A[l]
                    r-=1
            A[l]=flag
            if l==k-1:
                return A[l]
            elif l<k-1:
                l,r=l+1,r1
            else:
                l,r=l1,r-1
        
        return 0
                
        
