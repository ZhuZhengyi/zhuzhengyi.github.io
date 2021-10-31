#
# @lc app=leetcode.cn id=56 lang=python3
#
# [56] 合并区间
#
# https://leetcode-cn.com/problems/merge-intervals/description/
#
# algorithms
# Medium (39.40%)
# Likes:    235
# Dislikes: 0
# Total Accepted:    42K
# Total Submissions: 106.5K
# Testcase Example:  '[[1,3],[2,6],[8,10],[15,18]]'
#
# 给出一个区间的集合，请合并所有重叠的区间。
# 
# 示例 1:
# 
# 输入: [[1,3],[2,6],[8,10],[15,18]]
# 输出: [[1,6],[8,10],[15,18]]
# 解释: 区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].
# 
# 
# 示例 2:
# 
# 输入: [[1,4],[4,5]]
# 输出: [[1,5]]
# 解释: 区间 [1,4] 和 [4,5] 可被视为重叠区间。
# 
#

# @lc code=start
class Solution:
    def merge(self, intervals: List[List[int]]) -> List[List[int]]:
        if len(intervals) < 1:
            return []
        sorted_intervals = sorted(intervals, key=lambda s: s[0])
        res = []
        pending = sorted_intervals[0]
        for current in sorted_intervals[1:]:
            # [ pending ]
            #     [ current
            if current[0] <= pending[1]:
                pending[1] = max(current[1], pending[1]) 
            #  [ pending ]
            #               [ current ]
            else:
                res.append(pending)
                pending = current
        res.append(pending)
        return res
        
# @lc code=end

