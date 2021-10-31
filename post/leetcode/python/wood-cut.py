# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/wood-cut
@Language: Python
@Datetime: 15-08-09 13:30
'''

class Solution:
    """
    @param L: Given n pieces of wood with length L[i]
    @param k: An integer
    return: The maximum length of the small pieces.
    """
    def woodCut(self, L, k):
        # write your code here
        if sum(L) < k:
            return 0
            
        maxLen = max(L)
        start, end = 1, maxLen
        while start + 1 < end:
            mid = (start + end) / 2
            pieces = sum([l / mid for l in L])
            if pieces >= k:
                start = mid
            else:
                end = mid
                
        if sum([l / end for l in L]) >= k:
            return end
        return start
