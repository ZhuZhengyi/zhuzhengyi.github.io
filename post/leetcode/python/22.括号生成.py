#
# @lc app=leetcode.cn id=22 lang=python3
#
# [22] 括号生成
#
# https://leetcode-cn.com/problems/generate-parentheses/description/
#
# algorithms
# Medium (73.37%)
# Likes:    770
# Dislikes: 0
# Total Accepted:    77.7K
# Total Submissions: 105.8K
# Testcase Example:  '3'
#
# 给出 n 代表生成括号的对数，请你写出一个函数，使其能够生成所有可能的并且有效的括号组合。
# 
# 例如，给出 n = 3，生成结果为：
# 
# [
# ⁠ "((()))",
# ⁠ "(()())",
# ⁠ "(())()",
# ⁠ "()(())",
# ⁠ "()()()"
# ]
# 
# 
#

# @lc code=start
class Solution:
    def generateParenthesis(self, n: int) -> List[str]:
        def generate(p, l, r, ps):
            if l == 0 and r == 0:
                ps.append(p)
                return
            elif l > r: #
                return
            if l > 0:
                generate(p+'(', l-1, r, ps)
            if r > 0:
                generate(p+')', l, r-1, ps)

        parentheses = []
        generate("", n, n, parentheses)
        return parentheses
# @lc code=end

