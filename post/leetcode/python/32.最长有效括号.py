#
# @lc app=leetcode.cn id=32 lang=python3
#
# [32] 最长有效括号
#
# https://leetcode-cn.com/problems/longest-valid-parentheses/description/
#
# algorithms
# Hard (29.67%)
# Likes:    543
# Dislikes: 0
# Total Accepted:    44.1K
# Total Submissions: 147.6K
# Testcase Example:  '"(()"'
#
# 给定一个只包含 '(' 和 ')' 的字符串，找出最长的包含有效括号的子串的长度。
# 
# 示例 1:
# 
# 输入: "(()"
# 输出: 2
# 解释: 最长有效括号子串为 "()"
# 
# 
# 示例 2:
# 
# 输入: ")()())"
# 输出: 4
# 解释: 最长有效括号子串为 "()()"
# 
# 
#

# @lc code=start
class Solution:
    def longestValidParentheses(self, s: str) -> int:
        """
        ## 解题思路
        设 f[i]为以s[i]为尾的最长有效括号长度，此时
        * 如果s[i]是'('，则以其为结尾不是有效括号，因此f[i]为0;            
        * 如果s[i]是')'，以s[i-1]为尾的最长有效括号字符串为s[i-f[i-1]:i]。 
          示意图：   
                pre_index
                    |
                    v| f[i-1] |
                *****(********))
                     ^        ^
                  i-f[i-1]  (i-1)     
          其前一个字符为s[i-f[i-1]-1], 记 pre_index=i-f[i-1]-1
          - 如果s[pre_index]为'(', 则s[pre_index:i+1]也为有效括号串, 即:
                  f[i] = f[i-1] + 2     
          示意图：  ******((********))
                         ^          ^
                    pre_index       i
          - 此时，如果pre_index前仍然存在字符串，则最长有效长度还加上前面部分的有效长度，
          使前后连贯起来, 因此：
                    f[i] += f[pre_index-1] 
          示意图
              ***(***)((********))
                      ^
                  pre_index   
        """
        if len(s) < 2:
          return 0

        f = [ 0 ] * len(s)
        for i in range(1, len(s)):
          if s[i] == ')':             # 如果以')'结尾，则存在有效配对可能, ************)
            pre_last = i-f[i-1]-1     # 上一个最长有效配对前一个字符
            if pre_last >=0 and s[pre_last] == '(':  #可以匹配         *****((*****))
              f[i] = f[i-1] + 2       # 则在上一个有效配对长度上再加上()
              if pre_last > 0:        # 如果之前还有字符串，则要再加上该字符前的有效长度
                f[i] += f[pre_last-1] #  ****(***)((********))

        return max(f)

# @lc code=end

if __name__ == "__main__":
  s = Solution()
  tests = [ 
    ( "()(())", 6 ),
    ( "(()))())(", 4 ),
  ]
  for test in tests:
    print(test, s.longestValidParentheses(test[0]) == test[1] )

