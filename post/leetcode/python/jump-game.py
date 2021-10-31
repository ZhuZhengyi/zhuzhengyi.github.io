# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/jump-game
@Language: Python
@Datetime: 15-08-20 13:50
'''

class Solution:
    # @param A, a list of integers
    # @return a boolean
    def canJump(self, A):
        # write your code here
        n=len(A)
        maxN=0
        step=0
        while step<=maxN:
            maxN=max(maxN, step+A[step])
            if maxN>=n-1:
                return True
            step+=1

        return False
