# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/paint-fence
@Language: Python
@Datetime: 17-02-05 02:06
'''

class Solution:
    # @param {int} n non-negative integer, n posts
    # @param {int} k non-negative integer, k colors
    # @return {int} an integer, the total number of ways
    def numWays(self, n, k):
        # Write your code here
        #
        # record[i]: ç¬¬iæ ¹æ±å­çæè²æ¹æ¡
        # 1. åç¬¬i-1æ ¹æ±å­é¢è²ç¸å, ä¸åç¬¬i-2æ ¹æ±å­é¢è²ç¸å, æ­¤æ¶ç¬¬i-1,iæ ¹æ±å­å¯æk-1ç§é¢è²;
        # 2. åç¬¬i-1æ ¹æ±å­é¢è²ä¸å, ç¬¬iæ ¹æ±å­å¯æk-1ç§é¢è²ï¼
        # æ record[i] = record[i-2] * (k-1) + record[i-1] * (k-1)
        #
        if n == 0 or k == 0:
            return 0
            
        record = [k, k*k]
        if n == 1:
            return record[0]
        else:
            i=2
            while i < n:
                t = sum(record) * (k-1)
                record[0] = record[1]
                record[1] = t
                i += 1
        return record[1]