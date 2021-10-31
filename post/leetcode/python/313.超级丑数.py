#!/usr/bin/env python
#--*--coding:utf-8--*--
# @lc app=leetcode.cn id=313 lang=python3
#
# [313] 超级丑数
#
# https://leetcode-cn.com/problems/super-ugly-number/description/
#
# algorithms
# Medium (63.78%)
# Likes:    132
# Dislikes: 0
# Total Accepted:    12.4K
# Total Submissions: 19.4K
# Testcase Example:  '12\n[2,7,13,19]'
#
# 编写一段程序来查找第 n 个超级丑数。
# 
# 超级丑数是指其所有质因数都是长度为 k 的质数列表 primes 中的正整数。
# 
# 示例:
# 
# 输入: n = 12, primes = [2,7,13,19]
# 输出: 32 
# 解释: 给定长度为 4 的质数列表 primes = [2,7,13,19]，前 12
# 个超级丑数序列为：[1,2,4,7,8,13,14,16,19,26,28,32] 。
# 
# 说明:
# 
# 
# 1 是任何给定 primes 的超级丑数。
# 给定 primes 中的数字以升序排列。
# 0 < k ≤ 100, 0 < n ≤ 10^6, 0 < primes[i] < 1000 。
# 第 n 个超级丑数确保在 32 位有符整数范围内。
# 
# 
#

from typing import List

# @lc code=start
class Solution:
    def nthSuperUglyNumber(self, n: int, primes: List[int]) -> int:
        '''
        ## 解法一:
            堆
            将所有加入堆中，第n个丑数就是第n个

        '''
        if n < 1:
            return 0
        import heapq

        heap = [1]
        n -= 1
        while n:                
            
            pass

        return heapq.heappop(heap)


    def nthSuperUglyNumber2(self, n: int, primes: List[int]) -> int:
        '''
        ## 解法二：动态规划                        
            $$
                ugly_num = primes[0]^x * primes[1]^y
            $$
        '''
        if n < 1:
            return 0
        uglies = [0] * n
        uglies[0] = 1
        primes_to_uglies_loc = [0] * len(primes)

        for i in range(1, n):
            uglies[i] = min(x * uglies[y] for x, y in zip(primes, primes_to_uglies_loc))
            for j in range(len(primes)):
                if uglies[i] >= primes[j] * uglies[primes_to_uglies_loc[j]]:
                    primes_to_uglies_loc[j] += 1

        return uglies[-1]
        
# @lc code=end

if __name__ == "__main__":
    s = Solution()
    print(s.nthSuperUglyNumber(12, [2,7,13,19]))
