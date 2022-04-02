#
# @lc app=leetcode.cn id=4 lang=python3
#
# [4] 寻找两个有序数组的中位数
#
# https://leetcode-cn.com/problems/median-of-two-sorted-arrays/description/
#
# algorithms
# Hard (36.30%)
# Likes:    1798
# Dislikes: 0
# Total Accepted:    115.7K
# Total Submissions: 318.6K
# Testcase Example:  '[1,3]\n[2]'
#
# 给定两个大小为 m 和 n 的有序数组 nums1 和 nums2。
#
# 请你找出这两个有序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
#
# 你可以假设 nums1 和 nums2 不会同时为空。
#
# 示例 1:
#
# nums1 = [1, 3]
# nums2 = [2]
#
# 则中位数是 2.0
#
#
# 示例 2:
#
# nums1 = [1, 2]
# nums2 = [3, 4]
#
# 则中位数是 (2 + 3)/2 = 2.5
#
#          l   i   h
#  nums1:  * * * * *
#  nums2:  * * * * * * *
#            mid

# @lc code=start
class Solution:
    '''
    ## 解题思路
    * 
    '''
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        a, b = sorted((nums1, nums2), key=len)
        m, n = len(a), len(b)
        l, h = 0, m  
        mid = int((m + n - 1) / 2)  # mid for total nums1 and nums2
        while l < h:
            i = int((l + h) / 2)  # mid of nums1
            if i > mid - 1 or a[i] >= b[mid - 1 - i]:  # nums1 当前
                h = i  #
            else: 
                l = i + 1  #
        i = l
        nextfew = sorted(a[i:i+2] + b [mid-i: mid-i+2])
        return (nextfew[0] + nextfew[1-(m+n)%2]) / 2.0
# @lc code=end

