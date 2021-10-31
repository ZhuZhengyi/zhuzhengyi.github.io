# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/unique-characters
@Language: Python
@Datetime: 15-07-29 15:00
'''

class Solution:
    # @param s: a string
    # @return: a boolean
    def isUnique(self, str):
        # write your code here
        n = len(str)
        for i in range(n-2):
            for j in range(i+1, n):
                if str[i]==str[j]:
                    return False
                    
        return True
