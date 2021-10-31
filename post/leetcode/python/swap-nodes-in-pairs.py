# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/swap-nodes-in-pairs
@Language: Python
@Datetime: 17-02-05 08:12
'''

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    # @param head, a ListNode
    # @return a ListNode
    def swapPairs(self, head):
        # Write your code here
        
        r = ListNode(0)
        p1 = r
        p = head
        while p is not None and p.next is not None:
            p2 = p.next.next
            r.next = p.next; r = r.next
            r.next = p; r = r.next
            p = p2
            
        if p is not None:
            r.next = p
        else:
            r.next = None
        
        return p1.next