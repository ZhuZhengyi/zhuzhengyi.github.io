# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/remove-duplicates-from-sorted-list-ii
@Language: Python
@Datetime: 15-08-24 14:37
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
    @param head: A ListNode
    @return: A ListNode
    """
    def deleteDuplicates(self, head):
        # write your code here
        dummy=ListNode(0)
        h=dummy

        p=head
        while p!=None:
            # p is last node
            if p.next==None:
                h.next=p
                break
            # p is unique node 
            if p.next!=None and p.next.val!=p.val:
                h.next=p
                p=p.next
                h=h.next
                h.next=None
            # p has same node
            else:
                val=p.val
                p=p.next
                while p!=None and p.val==val:
                    p=p.next
        
        return dummy.next
                
                
            
        
