# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/merge-two-sorted-lists
@Language: Python
@Datetime: 15-07-25 23:12
'''

"""
Definition of ListNode
class ListNode(object):
    def __init__(self, val, next=None):
        self.val = val
        self.next = next
"""
class Solution:
    """
    @param two ListNodes
    @return a ListNode
    """
    def mergeTwoLists(self, l1, l2):
        # write your code here
        h=ListNode(0)
        t=h
        
        while l1!=None and l2!=None :
            if l1.val>l2.val:
                t.next = l2
                l2=l2.next
            else:
                t.next = l1
                l1=l1.next
            t=t.next
            
        t.next = l1 if l1!=None else l2
        
        return h.next

