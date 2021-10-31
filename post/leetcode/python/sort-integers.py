# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/sort-integers
@Language: Python
@Datetime: 17-01-26 07:15
'''

class Solution:
    # @param {int[]} A an integer array
    # @return nothing
    def sortIntegers(self, A):
        # Write your code here
        for i in range(1, len(A)):
            for j in range(0, len(A)-i):
                if A[j] > A[j+1]:
                    A[j+1] , A[j] = A[j], A[j+1]