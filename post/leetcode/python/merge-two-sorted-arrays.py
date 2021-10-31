# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/merge-two-sorted-arrays
@Language: Python
@Datetime: 15-07-26 08:10
'''

class Solution:
    #@param A and B: sorted integer array A and B.
    #@return: A new sorted integer array
    def mergeSortedArray(self, A, B):
        # write your code here
        res=[]
        i,j=0,0
        while i<len(A) and j<len(B):
            if A[i]<B[j]:
                res.append(A[i])
                i+=1
            else:
                res.append(B[j])
                j+=1
        while i<len(A):
            res.append(A[i])
            i+=1
        while j<len(B):
            res.append(B[j])
            j+=1
        return res
