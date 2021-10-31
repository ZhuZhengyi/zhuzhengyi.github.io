#
# @lc app=leetcode.cn id=64 lang=python3
#
# [64] 最小路径和
#
# https://leetcode-cn.com/problems/minimum-path-sum/description/
#
# algorithms
# Medium (63.17%)
# Likes:    337
# Dislikes: 0
# Total Accepted:    45.7K
# Total Submissions: 72.1K
# Testcase Example:  '[[1,3,1],[1,5,1],[4,2,1]]'
#
# 给定一个包含非负整数的 m x n 网格，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。
# 
# 说明：每次只能向下或者向右移动一步。
# 
# 示例:
# 
# 输入:
# [
# [1,3,1],
# ⁠ [1,5,1],
# ⁠ [4,2,1]
# ]
# 输出: 7
# 解释: 因为路径 1→3→1→1→1 的总和最小。
# 
# 
#

# @lc code=start
class Solution:
    def minPathSum(self, grid: List[List[int]]) -> int:
        m=len(grid)     #m
        if m<1:
            return 0
        if m==1:
            return sum(grid[0])
        
        n=len(grid[0])  #n
        if n<1:
            return 0
        
        path=[[grid[0][0]]*n for _ in range(m) ]
        for i in range(m):      #i
            for j in range(n):  #j
                if i==0 and j==0:
                    path[i][j]=grid[i][j]
                elif i==0 and j>0:
                    path[i][j]=path[i][j-1]+grid[i][j]
                elif j==0 and i>0:
                    path[i][j]=path[i-1][j]+grid[i][j]
                else:
                    path[i][j]=min(path[i][j-1], path[i-1][j])+grid[i][j]
                    
        return path[m-1][n-1]
        
# @lc code=end

