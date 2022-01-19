#
# @lc app=leetcode.cn id=42 lang=python3
#
# [42] 接雨水
#
# https://leetcode-cn.com/problems/trapping-rain-water/description/
#
# algorithms
# Hard (47.44%)
# Likes:    747
# Dislikes: 0
# Total Accepted:    43.1K
# Total Submissions: 90.6K
# Testcase Example:  '[0,1,0,2,1,0,1,3,2,1,2,1]'
#
# 给定 n 个非负整数表示每个宽度为 1 的柱子的高度图，计算按此排列的柱子，下雨之后能接多少雨水。
# 
# 
# 
# 上面是由数组 [0,1,0,2,1,0,1,3,2,1,2,1] 表示的高度图，在这种情况下，可以接 6 个单位的雨水（蓝色部分表示雨水）。 感谢
# Marcos 贡献此图。
# 
# 示例:
# 
# 输入: [0,1,0,2,1,0,1,3,2,1,2,1]
# 输出: 6
# 
#

# @lc code=start
class Solution:
    def trap(self, height: List[int]) -> int:
        '''
        ## 解题思路
        * 双指针法
          1. 设置l,r 指针，分别指向数组的两端；
          2. 根据当前l,r所指柱子的高度，选择小的柱子lower_height往里移动；
          3. 记录下移动过的柱子最大高度edge_height；
          4. 每次移动时，可收集到的雨水量为`edge_height - lower_height`
          5. 总雨水长度为
        '''
        l, r = 0, len(height)-1
        total = 0       #总水量
        cur_height = 0
        edge_heigh = 0
        while l < r:
            if height[l] < height[r]:
                cur_height = height[l]
                l+=1
            else:
                cur_height = height[r]
                r-=1
            edge_heigh = max(edge_heigh, cur_height)
            total += edge_heigh - cur_height

        return total 
# @lc code=end

