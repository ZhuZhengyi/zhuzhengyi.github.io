# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/insert-interval
@Language: Python
@Datetime: 15-08-03 12:20
'''

"""
Definition of Interval.
class Interval(object):
    def __init__(self, start, end):
        self.start = start
        self.end = end
"""


class Solution:
    """
    Insert a new interval into a sorted non-overlapping interval list.
    @param intevals: Sorted non-overlapping interval list
    @param newInterval: The new interval.
    @return: A new sorted non-overlapping interval list with the new interval.
    """
    def insert(self, intervals, newInterval):
        # write your code here
        n=len(intervals)
        if n<1:
            intervals.append(newInterval)
            return intervals
        
        # end
        if newInterval.start > intervals[-1].end:
            intervals.append(newInterval)
            return intervals
        
        i=0
        while i<len(intervals):
            #  [1,2]
            #        [3,4]
            if newInterval.start > intervals[i].end:
                i+=1
                continue
            #         [5,9] 
            #  [3,4]
            elif newInterval.end<intervals[i].start:
                intervals.insert(i,newInterval)
                i+=1
                break
            #   [ 2,   5 ]         
            #     [3, 4]
            elif newInterval.start>=intervals[i].start and newInterval.end<=intervals[i].end:
                i+=1
                continue
            #   [2, 5]  [7, 10]
            #  [          ,8]
            elif i<len(intervals)-1 and newInterval.end>=intervals[i+1].start:
                newInterval.start=min(newInterval.start, intervals[i].start)
                intervals.pop(i)
            #    [2, 5]   
            #   [   ]
            else:
                intervals[i].start=min(newInterval.start, intervals[i].start)
                intervals[i].end=max(newInterval.end, intervals[i].end)
                i+=1
                break
        
        return intervals
