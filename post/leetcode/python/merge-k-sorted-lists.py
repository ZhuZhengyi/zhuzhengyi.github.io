# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/merge-k-sorted-lists
@Language: Python
@Datetime: 15-08-17 14:00
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
    @param lists: a list of ListNode
    @return: The head of one sorted list.
    """
    def mergeKLists(self, lists):
        # write your code here
        n=len(lists)
        if n==0:
            return None
            
        if n==1:
            return lists[0]
            
        h=l=ListNode(0)
        if n==2:
            l1,l2=lists[0],lists[1]
            while l1!=None and l2!=None:
                if l1.val<l2.val:
                    l.next=l1
                    l1=l1.next
                else:
                    l.next=l2
                    l2=l2.next
                l=l.next
            l.next=l1 if l1!=None else l2
            return h.next
                       
        l1=self.mergeKLists(lists[:n/2])
        l2=self.mergeKLists(lists[n/2:])
        l=self.mergeKLists([l1,l2])

        return l

