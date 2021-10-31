# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/ugly-number
@Language: Python
@Datetime: 17-01-21 10:40
'''

class Solution:
    # @param {int} num an integer
    # @return {boolean} true if num is an ugly number or false
    def isUgly(self, num):
        # Write your code here
        if num == 0:
            return False
        elif num == 1 or num == 2 or num == 3 or num == 5:
            return True
        elif num % 2 == 0:
            return self.isUgly(num/2)
        elif num % 3  == 0:
            return self.isUgly(num/3)
        elif num % 5 == 0:
            return self.isUgly(num/5)
        else:
            return False