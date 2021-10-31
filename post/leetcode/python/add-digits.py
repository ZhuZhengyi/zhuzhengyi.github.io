# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/add-digits
@Language: Python
@Datetime: 17-01-26 07:42
'''

class Solution:
    # @param {int} num a non-negative integer
    # @return {int} one digit
    def addDigits(self, num):
        # Write your code here
        if len(str(num)) == 1:
            return num
        else:
            return self.addDigits(sum([int(i) for i in list(str(num))]))