# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/intersection-of-two-arrays-ii
@Language: Python
@Datetime: 17-01-01 02:48
'''

class Solution:
    # @param {int[]} nums1 an integer array
    # @param {int[]} nums2 an integer array
    # @return {int[]} an integer array
    def intersection(self, nums1, nums2):
        # Write your code here
        l1, l2 = len(nums1), len(nums2)
        if l1 <= 0 or l2 <=0:
            return []
            
        nums1.sort()
        nums2.sort()
        
        res = []
        i1, i2 = 0, 0
        while i1<l1 and i2<l2:
            if nums1[i1] == nums2[i2]:
                res.append(nums1[i1])
                i1 += 1
                i2 += 1
            elif nums1[i1] < nums2[i2]:
                i1 += 1
            else:
                i2 += 1
        
        return res