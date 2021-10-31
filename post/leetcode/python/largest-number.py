# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/largest-number
@Language: Python
@Datetime: 15-08-21 15:05
'''

class Solution:	
    #@param num: A list of non negative integers
    #@return: A string
    def largestNumber(self, num):
        # write your code here
        if sum(num)==0:
            return "0"
        
        strNum=[ str(n) for n in num ]
        
        strNum.sort(cmp)

        return "".join(strNum[::-1])
        
def cmp(s1,s2):
    n1,n2=len(s1),len(s2)
    i=0
    while i<min(n1,n2):
        if s1[i]>s2[i]:
            return 1
        elif s1[i]<s2[i]:
            return -1
        i+=1
        
    if i>n1-1 and i<n2:
        if s2[i]>s2[0]:
            return -1
        else:
            return 1
    if i>n2-1 and i<n1:
        if s1[i]>s1[0]:
            return 1
        else:
            return -1
    
    return 1
        
