# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/digit-counts
@Language: Python
@Datetime: 17-02-10 15:04
'''

class Solution:
    # @param k & n  two integer
    # @return ans a integer
    def digitCounts(self, k, n):
        count = 0
        for i in range(n+1):
            while i > 9:
                if i%10 == k:
                    count += 1
                i = i/10
            if i == k:
                count += 1
        return count
