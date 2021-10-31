#
# @lc app=leetcode.cn id=57 lang=python3
#
# [57] 插入区间
#
# https://leetcode-cn.com/problems/insert-interval/description/
#
# algorithms
# Hard (36.07%)
# Likes:    84
# Dislikes: 0
# Total Accepted:    12.3K
# Total Submissions: 34.2K
# Testcase Example:  '[[1,3],[6,9]]\n[2,5]'
#
# 给出一个无重叠的 ，按照区间起始端点排序的区间列表。
# 
# 在列表中插入一个新的区间，你需要确保列表中的区间仍然有序且不重叠（如果有必要的话，可以合并区间）。
# 
# 示例 1:
# 
# 输入: intervals = [[1,3],[6,9]], newInterval = [2,5]
# 输出: [[1,5],[6,9]]
# 
# 
# 示例 2:
# 
# 输入: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
# 输出: [[1,2],[3,10],[12,16]]
# 解释: 这是因为新的区间 [4,8] 与 [3,5],[6,7],[8,10] 重叠。
# 
# 
#

# @lc code=start
class Solution:
    def insert(self, intervals: List[List[int]], newInterval: List[int]) -> List[List[int]]:
        if len(intervals) == 0:
            return [ newInterval ]
        intervals.append(newInterval)
        if len(intervals) < 1:
            return []
        sorted_intervals = sorted(intervals, key=lambda s: s[0])

        pending = sorted_intervals[0]
        res = []
        for current in sorted_intervals[1:]:
            if current[0] <= pending[1]:
                pending[1] = max(current[1], pending[1]) 
            else:
                res.append(pending)
                pending = current
        res.append(pending)

        return res
        
# @lc code=end

