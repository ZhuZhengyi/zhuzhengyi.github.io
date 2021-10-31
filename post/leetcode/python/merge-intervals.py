# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/merge-intervals
@Language: Python
@Datetime: 15-08-23 04:05
'''



"""
Definition of Interval.
class Interval(object):
    def __init__(self, start, end):
        self.start = start
        self.end = end
"""

class Solution:
    # @param intervals, a list of Interval
    # @return a list of Interval
    def merge(self, intervals):
        # write your code here
        intervals.sort(cmp=lambda x,y:cmp(x.start,y.start))
        
        n=len(intervals)
        if n<=1:
            return intervals
        
        i,j=0,1
        while i<len(intervals):
            j=i+1
            while j<len(intervals):
                if intervals[j].start<=intervals[i].end:
                    intervals[i].end=max(intervals[i].end, intervals[j].end)
                    intervals.pop(j)
                else:
                    j+=1
            i+=1
        return intervals