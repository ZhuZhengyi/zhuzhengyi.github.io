# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/classical-binary-search
@Language: Python
@Datetime: 17-02-05 10:34
'''

class Solution:
    # @param {int[]} A an integer array sorted in ascending order
    # @param {int} target an integer
    # @return {int} an integer
    def findPosition(self, A, target):
        # Write your code here
        if len(A) < 1 or target < A[0] or target > A[-1]:
            return -1

        l, r = 0, len(A)-1
        while l < r:
            m = (l + r) >> 1
            if A[m] == target:
                return m
            elif A[m] > target:
                r = m
            else:
                l = m+1
        return -1

