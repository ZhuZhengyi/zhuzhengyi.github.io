#
# @lc app=leetcode.cn id=55 lang=python3
#
# [55] 跳跃游戏
#
# https://leetcode-cn.com/problems/jump-game/description/
#
# algorithms
# Medium (37.11%)
# Likes:    392
# Dislikes: 0
# Total Accepted:    47.7K
# Total Submissions: 128.6K
# Testcase Example:  '[2,3,1,1,4]'
#
# 给定一个非负整数数组，你最初位于数组的第一个位置。
# 
# 数组中的每个元素代表你在该位置可以跳跃的最大长度。
# 
# 判断你是否能够到达最后一个位置。
# 
# 示例 1:
# 
# 输入: [2,3,1,1,4]
# 输出: true
# 解释: 我们可以先跳 1 步，从位置 0 到达 位置 1, 然后再从位置 1 跳 3 步到达最后一个位置。
# 
# 
# 示例 2:
# 
# 输入: [3,2,1,0,4]
# 输出: false
# 解释: 无论怎样，你总会到达索引为 3 的位置。但该位置的最大跳跃长度是 0 ， 所以你永远不可能到达最后一个位置。
# 
# 
#

# @lc code=start
class Solution:
    def canJump(self, nums: List[int]) -> bool:
        '''
        解法：
            贪心法。
            从左到右遍历序列，用一个数记录已经遍历过的最远距离max_jumped。
            如果当前位置到达，且能跳过超过max_jumped, 则更新max_jumped;
            最后判断max_jumped是否到最后位置 
        '''
        max_jumped = 0
        for i, jump in enumerate(nums):
            #能达到当前位置, 且
            if max_jumped >= i and i + jump > max_jumped:
                max_jumped = i + jump
        return max_jumped >= len(nums) - 1
        
# @lc code=end

