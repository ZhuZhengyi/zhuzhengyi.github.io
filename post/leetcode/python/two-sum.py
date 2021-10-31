# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/two-sum
@Language: Python
@Datetime: 15-08-05 12:50
'''

class Solution:
    """
    @param numbers : An array of Integer
    @param target : target = numbers[index1] + numbers[index2]
    @return : [index1 + 1, index2 + 1] (index1 < index2)
    """
    def twoSum(self, numbers, target):
        # write your code here
        n=len(numbers)

        for i in range(n):
            for j in range(i+1, n):
                if numbers[i]+numbers[j]==target:
                    return [i+1,j+1]

        return None
