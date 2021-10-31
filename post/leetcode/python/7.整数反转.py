#
# @lc app=leetcode.cn id=7 lang=python3
#
# [7] 整数反转
#
# https://leetcode-cn.com/problems/reverse-integer/description/
#
# algorithms
# Easy (33.23%)
# Likes:    1485
# Dislikes: 0
# Total Accepted:    228.5K
# Total Submissions: 687.7K
# Testcase Example:  '123'
#
# 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。
# 
# 示例 1:
# 
# 输入: 123
# 输出: 321
# 
# 
# 示例 2:
# 
# 输入: -123
# 输出: -321
# 
# 
# 示例 3:
# 
# 输入: 120
# 输出: 21
# 
# 
# 注意:
# 
# 假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−2^31,  2^31 − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。
# 
#

# @lc code=start
class Solution:
    def reverse(self, x: int) -> int:
        if x > (1<<31)-1:
            return 0
        if x < -1*(1<<31):
            return 0
        X = x if x >0 else -x
        
        flag = 1 if x >0 else -1
        y = 0
        while X > 0:
            m = X % 10
            y = y * 10 + m
            X = int(X / 10)
        if y > (1<<31) -1:
            return 0
        if y < -1*(1<<31):
            return 0
        return  y if flag > 0 else -y 
        
        
# @lc code=end

