#!/usr/bin/env python
#-*-coding:utf-8-*- 
# @lc app=leetcode.cn id=295 lang=python3
#
# [295] 数据流的中位数
#
# https://leetcode-cn.com/problems/find-median-from-data-stream/description/
#
# algorithms
# Hard (49.17%)
# Likes:    309
# Dislikes: 0
# Total Accepted:    25.7K
# Total Submissions: 52.2K
# Testcase Example:  '["MedianFinder","addNum","addNum","findMedian","addNum","findMedian"]\n' +
#  '[[],[1],[2],[],[3],[]]'
#
# 中位数是有序列表中间的数。如果列表长度是偶数，中位数则是中间两个数的平均值。
#
# 例如，
#
# [2,3,4] 的中位数是 3
#
# [2,3] 的中位数是 (2 + 3) / 2 = 2.5
#
# 设计一个支持以下两种操作的数据结构：
#
#
# void addNum(int num) - 从数据流中添加一个整数到数据结构中。
# double findMedian() - 返回目前所有元素的中位数。
#
#
# 示例：
#
# addNum(1)
# addNum(2)
# findMedian() -> 1.5
# addNum(3)
# findMedian() -> 2
#
# 进阶:
#
#
# 如果数据流中所有整数都在 0 到 100 范围内，你将如何优化你的算法？
# 如果数据流中 99% 的整数都在 0 到 100 范围内，你将如何优化你的算法？
#
#
#

# @lc code=start
import heapq

class MedianFinder:
    '''
    ## 解法:
    * 双堆法
    1. 建立左右两个堆，左为大顶堆，右为小顶堆;
    2. 每次先将元素取反, push大顶堆;
    3. 再将大顶堆顶元素弹出, 取反后push到小顶堆;
    4. 如果元素总个数为奇数，再将小顶堆的堆顶元素弹出，取反后push到大顶堆；
    '''

    def __init__(self):
        """
        initialize your data structure here.
        """
        self.count = 0
        self.l_maxheap, self.r_minheap = [], []

    def addNum(self, num: int) -> None:
        self.count += 1
        heapq.heappush(self.l_maxheap, -num)
        heapq.heappush(self.r_minheap, -heapq.heappop(self.l_maxheap))
        if self.count & 1:
            heapq.heappush(self.l_maxheap, -heapq.heappop(self.r_minheap))


    def findMedian(self) -> float:
        if self.count & 1:
            return -self.l_maxheap[0]
        else:
            return (float(-self.l_maxheap[0]) + float(self.r_minheap[0]) ) / 2

# Your MedianFinder object will be instantiated and called as such:
# obj = MedianFinder()
# obj.addNum(num)
# param_2 = obj.findMedian()
# @lc code=end

if __name__ == "__main__":
    s = MedianFinder()
    s.addNum(1)
    print(s.l_maxheap, s.r_minheap)
    print(s.findMedian() == 1)
    s.addNum(2)
    print(s.l_maxheap, s.r_minheap)
    print(s.findMedian() == 1.5)
