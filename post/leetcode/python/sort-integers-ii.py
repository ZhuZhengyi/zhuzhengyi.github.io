# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/sort-integers-ii
@Language: Python
@Datetime: 17-03-14 14:27
'''

class Solution:
    # @param {int[]} A an integer array
    # @return nothing
    def sortIntegers2(self, A):
        # Write your code here
        if len(A) <= 1:
            return
        self.quickSort(A, 0, len(A)-1)
        
    def quickSort(self, A, l, r):
        if l<r:
            idx = self.partition(A, l, r)
            self.quickSort(A, l, idx-1)
            self.quickSort(A, idx+1, r)
    
    def partition(self, A, l, r):
        flag = A[l]
        while l<r:
            while r>l and A[r] > flag:
                r -= 1
            if r > l:
                A[l] = A[r]
                l += 1
            while l<r and A[l] < flag:
                l += 1
            if l < r:
                A[r] = A[l]
                r -= 1
        A[l] = flag
        return l
        
        
        
        
        
    
        