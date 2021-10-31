# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/segment-tree-build
@Language: Python
@Datetime: 17-03-04 07:15
'''

"""
Definition of SegmentTreeNode:
class SegmentTreeNode:
    def __init__(self, start, end):
        self.start, self.end = start, end
        self.left, self.right = None, None
"""

class Solution:	
    # @param start, end: Denote an segment / interval
    # @return: The root of Segment Tree
    def build(self, start, end):
        # write your code here
        if end < start:
            return None
        root = SegmentTreeNode(start, end)
        if end > start:
            root.left = self.build(start, start + (end - start) / 2)
            root.right = self.build(start + (end - start)/2 + 1, end)
        return root