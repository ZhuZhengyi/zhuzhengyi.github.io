# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/3sum-closest
@Language: Python
@Datetime: 15-08-09 03:36
'''

class Solution:
    """
    @param numbers: Give an array numbers of n integer
    @param target : An integer
    @return : return the sum of the three integers, the sum closest target.
    """
    def threeSumClosest(self, numbers, target):
        # write your code here
        n=len(numbers)
        if n < 3:
            return []

        numbers.sort()
        
        res=sum(numbers[:3])
        c=abs(target-res)
        t=c
        for i in range(n-2):
            if i>0 and numbers[i]==numbers[i-1]:
                continue
            j,k=i+1,n-1
            while j<k:
                s=numbers[i]+numbers[j]+numbers[k]
                t=abs(target-s)
                if j<n-2 and abs(target-(numbers[i]+numbers[j+1]+numbers[k]))<=t:
                    j+=1
                    continue
                if k>j+1 and abs(target-(numbers[i]+numbers[j]+numbers[k-1]))<=t:
                    k-=1
                    continue
                if t<c:
                    res=s
                    c=t
                break

        return res