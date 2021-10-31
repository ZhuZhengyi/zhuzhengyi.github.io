# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/find-minimum-in-rotated-sorted-array
@Language: Python
@Datetime: 15-08-09 13:37
'''

class Solution:
    # @param num: a rotated sorted array
    # @return: the minimum number in the array
    def findMin(self, num):
        # write your code here
        
        l=len(num)
        
        for i in range(l):
            if num[i]<num[(i+1)%l] and num[i]<num[(i+l-1)%l]:
                return num[i]
        
        return 0
