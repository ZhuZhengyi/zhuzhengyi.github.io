# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/left-pad
@Language: Python
@Datetime: 17-01-21 10:34
'''

class StringUtils:
    # @param {string} originalStr the string we want to append to
    # @param {int} size the target length of the string
    # @param {string} padChar the character to pad to the left side of the string
    # @return {string} a string
    @classmethod
    def leftPad(self, originalStr, size, padChar=' '):
        # Write your code here
        if len(originalStr) >= size:
            return originalStr
        else:
            prefix = ""
            prefix_len = size - len(originalStr)
            while prefix_len > 0:
                prefix += padChar
                prefix_len -= 1
            return prefix + originalStr
        