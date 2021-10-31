#
# @lc app=leetcode.cn id=507 lang=python3
#
# [507] 完美数
#
# https://leetcode-cn.com/problems/perfect-number/description/
#
# algorithms
# Easy (37.67%)
# Likes:    51
# Dislikes: 0
# Total Accepted:    11.4K
# Total Submissions: 30.3K
# Testcase Example:  '28'
#
# 对于一个 正整数，如果它和除了它自身以外的所有正因子之和相等，我们称它为“完美数”。
# 
# 给定一个 整数 n， 如果他是完美数，返回 True，否则返回 False
# 
# 
# 
# 示例：
# 
# 输入: 28
# 输出: True
# 解释: 28 = 1 + 2 + 4 + 7 + 14
# 
# 
# 
# 
# 提示：
# 
# 输入的数字 n 不会超过 100,000,000. (1e8)
# 
#

# @lc code=start
class Solution:
    def checkPerfectNumber(self, num: int) -> bool:
        if num <= 4:
            return False
        sons = self.getNumberSons(num)
        return num == sum(sons)

    def getNumberSons(self, num: int) -> []:
        if num == 1:
            return [1]
        if num <= 3:
            return [1, num]
        i, j = 2, num
        res = [1]
        while i < j:
            if num % i == 0:
                res.append(i)
                res.append(num/i)
            i += 1
            j = num / i

        return res
                
# @lc code=end

