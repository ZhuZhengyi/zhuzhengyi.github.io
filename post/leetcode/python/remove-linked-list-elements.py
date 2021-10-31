# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/remove-linked-list-elements
@Language: Python
@Datetime: 17-02-05 07:22
'''

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    # @param head, a ListNode
    # @param val, an integer
    # @return a ListNode
    def removeElements(self, head, val):
        # Write your code here
        p = ListNode(0)
        p.next = head
        p2 = p
        while p2.next is not None:
            if p2.next.val == val:
                if p.next == p2.next:
                    p = p.next
                p2.next = p2.next.next
                
            else:
                p2 = p2.next
        return p.next