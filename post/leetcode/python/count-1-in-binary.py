# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/count-1-in-binary
@Language: Python
@Datetime: 15-08-23 03:30
'''

class Solution:
    # @param num: an integer
    # @return: an integer, the number of ones in num
    def countOnes1(self, num):
        # write your code here
        r=0
        while num!=0:
            if num&1:
                r+=1
            num=num>>1
        return r

    def countOnes(self, num):
        # write your code here
        r=0
        while num!=0:
            num=num&(num-1)
            r+=1
        return r
