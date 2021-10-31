#
# @lc app=leetcode.cn id=2 lang=python3
#
# [2] 两数相加
#

# @lc code=start
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        p = dummy = ListNode(0)
        a = 0
        while l1 != None or l2 != None:
            v = a + (l1.val if l1!=None else 0) + (l2.val if l2!=None else 0)
            a = int(v/10)
            v = v - a*10
            p.next = ListNode(v)
            p = p.next
            if l1 != None:
                l1 = l1.next
            if l2 != None:
                l2 = l2.next

        if a > 0:
            p.next = ListNode(a)

        return dummy.next        

# @lc code=end

