# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/anagrams
@Language: Python
@Datetime: 15-07-19 13:48
'''

class Solution:
    # @param strs: A list of strings
    # @return: A list of strings
    def anagrams(self, strs):
        # write your code here
        astrs=[]
        str_dic={}
        for str1 in strs:
            l1=list(str1)
            l1.sort()
            s1="".join(l1)
            if s1 in str_dic:
                str_dic[s1]+=1
            else :
                str_dic[s1]=1
            
        for str1 in strs:
            l=list(str1)
            l.sort()
            s="".join(l)
            if str_dic[s]>1:
                astrs.append(str1)
                
        return astrs
                
            