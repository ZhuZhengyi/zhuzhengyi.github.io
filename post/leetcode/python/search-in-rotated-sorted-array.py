# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/search-in-rotated-sorted-array
@Language: Python
@Datetime: 17-02-26 10:50
'''

class Solution:
    """
    @param A : a list of integers
    @param target : an integer to be searched
    @return : an integer
    """
    def search(self, A, target):
        # write your code here
        l, r = 0, len(A)
        while l < r:
            m = l + (r - l - 1) / 2
            if A[m] == target:
                return m
            elif A[l] == target:
                return l
            elif A[r - 1] == target:
                return r - 1
            elif A[l] < A[r-1]:
                if A[m] > target:
                    r = m
                else:
                    l = m + 1
            else:
                if A[m] > A[l]:
                    if target > A[l] and target < A[m]:
                        r = m
                    else:
                        l = m + 1
                else:
                    if target < A[r-1] and target > A[m]:
                        l = m + 1
                    else:
                        r = m
        return -1