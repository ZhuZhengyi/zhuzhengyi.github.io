# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/search-in-rotated-sorted-array-ii
@Language: Python
@Datetime: 17-02-26 14:55
'''

class Solution:
    """
    @param A : an integer ratated sorted array and duplicates are allowed
    @param target : an integer to be searched
    @return : a boolean
    """
    def search(self, A, target):
        # write your code here
        l, r = 0, len(A)-1
        while l <= r:
            m = l + (r - l) / 2
            if A[m] == target:
                return True
            elif A[l] == target:
                return True
            elif A[r] == target:
                return True
            elif A[l] == A[r]:
                l += 1
                continue
            elif A[l] < A[r]:
                if A[m] > target:
                    r = m - 1
                else:
                    l = m + 1
            else:
                if A[m] > A[l]:
                    if target > A[l] and target < A[m]:
                        r = m - 1
                    else:
                        l = m + 1
                else:
                    if target < A[r] and target > A[m]:
                        l = m + 1
                    else:
                        r = m - 1
        return False
        