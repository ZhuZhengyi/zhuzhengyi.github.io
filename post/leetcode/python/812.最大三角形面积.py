#
# @lc app=leetcode.cn id=812 lang=python3
#
# [812] 最大三角形面积
#
# https://leetcode-cn.com/problems/largest-triangle-area/description/
#
# algorithms
# Easy (61.70%)
# Likes:    84
# Dislikes: 0
# Total Accepted:    11K
# Total Submissions: 17.4K
# Testcase Example:  '[[0,0],[0,1],[1,0],[0,2],[2,0]]'
#
# 给定包含多个点的集合，从其中取三个点组成三角形，返回能组成的最大三角形的面积。
# 
# 
# 示例:
# 输入: points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
# 输出: 2
# 解释: 
# 这五个点如下图所示。组成的橙色三角形是最大的，面积为2。
# 
# 
# 
# 
# 注意: 
# 
# 
# 3 <= points.length <= 50.
# 不存在重复的点。
# -50 <= points[i][j] <= 50.
# 结果误差值在 10^-6 以内都认为是正确答案。
# 
# 
#

# @lc code=start
class Solution:
    def largestTriangleArea(self, points: List[List[int]]) -> float:
        n = len(points)
        s = 0
        for i in range(1, n-2):
            for j in range(i+1, n-1):
                for k in range(j+1, n):
                    (x1, y1) = points[i]
                    (x2, y2) = points[j]
                    (x3, y3) = points[k]
                    s1 = abs( (x2-x1)*(y2-y1) - (x3-x1) * (y3-y1) )
                    s = max(s, s1)
                    
        return s/2

# @lc code=end

