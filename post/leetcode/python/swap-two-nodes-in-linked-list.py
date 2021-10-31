# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/swap-two-nodes-in-linked-list
@Language: Python
@Datetime: 17-01-29 10:58
'''

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    # @param {ListNode} head, a ListNode
    # @oaram {int} v1 an integer
    # @param {int} v2 an integer
    # @return {ListNode} a new head of singly-linked list
    def swapNodes(self, head, v1, v2):
        # Write your code here
        if head is None:
            return head
        phead = ListNode(0)
        phead.next = head
        p1p = p2p = phead
        while p1p.next is not None and p1p.next.val != v1:
            p1p = p1p.next
        while p2p.next is not None and p2p.next.val != v2:
            p2p = p2p.next
        p1 = p1p.next
        p2 = p2p.next
        if p1 is not None and p2 is not None:
            # æä¸èç¹ä¸ºé¦èç¹
            if phead.next == p1:
                phead.next = p2
            elif phead.next == p2:
                phead.next = p1
            p1a = p1.next
            p2a = p2.next
            # èç¹ç¸è¿
            if p1.next == p2:
                p1p.next = p2
                p2.next = p1
                p1.next = p2a
            elif p2.next == p1:
                p2p.next = p1
                p1.next = p2
                p2.next = p1a
            # å¶ä»æåµ
            else:
                p1p.next = p2
                p2.next = p1a
                p2p.next = p1
                p1.next = p2a
            
        return phead.next
