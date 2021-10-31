# coding:utf-8
'''
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/3sum
@Language: Python
@Datetime: 15-08-09 02:38
'''

class Solution:
    """
    @param numbersbers : Give an array numbersbers of n integer
    @return : Find all unique triplets in the array which gives the sum of zero.
    """
    def threeSum(self, numbers):
        # write your code here
        n=len(numbers)
        if n < 3:
            return []
        
        numbers.sort()
        res=[]
        """
        for i in range(n-2):
            if i>0 and numbers[i]==numbers[i-1]:
                    continue
            for j in range(i+1, n-1):
                if j>i+1 and numbers[j]==numbers[j-1]:
                    continue
                for k in range(j+1, n):
                    if k>j+1 and numbers[k]==numbers[k-1]:
                        continue
                    if numbers[i]+numbers[j]+numbers[k]==0:
                        res.append([numbers[i],numbers[j],numbers[k]])
        """
        
        for i in range(n-2):
            if i>0 and numbers[i]==numbers[i-1]:
                continue
            j,k=i+1,n-1
            while j<k:
                if j>i+1 and numbers[j]==numbers[j-1]:
                    j+=1
                    continue
                if k<n-1 and numbers[k]==numbers[k+1]:
                    k-=1
                    continue
                
                s=numbers[i]+numbers[j]+numbers[k]
            
                if s==0:
                    res.append([numbers[i],numbers[j],numbers[k]])
                    j+=1
                    k-=1
                elif s<0:
                    j+=1
                else:
                    k-=1
        
        return res
                
            
            
