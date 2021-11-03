# coding:utf-8
#
# @lc app=leetcode.cn id=1553 lang=python3
#
# [1553] 吃掉 N 个橘子的最少天数
#
# https://leetcode-cn.com/problems/minimum-number-of-days-to-eat-n-oranges/description/
#
# algorithms
# Hard (26.35%)
# Likes:    96
# Dislikes: 0
# Total Accepted:    7.8K
# Total Submissions: 26.3K
# Testcase Example:  '10'
#
# 厨房里总共有 n 个橘子，你决定每一天选择如下方式之一吃这些橘子：
# 
# 
# 吃掉一个橘子。
# 如果剩余橘子数 n 能被 2 整除，那么你可以吃掉 n/2 个橘子。
# 如果剩余橘子数 n 能被 3 整除，那么你可以吃掉 2*(n/3) 个橘子。
# 
# 
# 每天你只能从以上 3 种方案中选择一种方案。
# 
# 请你返回吃掉所有 n 个橘子的最少天数。
# 
# 
# 
# 示例 1：
# 
# 输入：n = 10
# 输出：4
# 解释：你总共有 10 个橘子。
# 第 1 天：吃 1 个橘子，剩余橘子数 10 - 1 = 9。
# 第 2 天：吃 6 个橘子，剩余橘子数 9 - 2*(9/3) = 9 - 6 = 3。（9 可以被 3 整除）
# 第 3 天：吃 2 个橘子，剩余橘子数 3 - 2*(3/3) = 3 - 2 = 1。
# 第 4 天：吃掉最后 1 个橘子，剩余橘子数 1 - 1 = 0。
# 你需要至少 4 天吃掉 10 个橘子。
# 
# 
# 示例 2：
# 
# 输入：n = 6
# 输出：3
# 解释：你总共有 6 个橘子。
# 第 1 天：吃 3 个橘子，剩余橘子数 6 - 6/2 = 6 - 3 = 3。（6 可以被 2 整除）
# 第 2 天：吃 2 个橘子，剩余橘子数 3 - 2*(3/3) = 3 - 2 = 1。（3 可以被 3 整除）
# 第 3 天：吃掉剩余 1 个橘子，剩余橘子数 1 - 1 = 0。
# 你至少需要 3 天吃掉 6 个橘子。
# 
# 
# 示例 3：
# 
# 输入：n = 1
# 输出：1
# 
# 
# 示例 4：
# 
# 输入：n = 56
# 输出：6
# 
# 
# 
# 
# 提示：
# 
# 
# 1 <= n <= 2*10^9
# 
# 
#

# @lc code=start
class Solution:
    # def minDays(self, n: int) -> int:
    #     if n < 2:
    #         return n

    #     res = [ 0, 1 ]
    #     for i in range(2, n+1):
    #         res.append( min(res[int(i/2)]+i%2, res[int(i/3)] + i%3 ) + 1)

    #     return res[n]

    def minDays(self, n: int) -> int:
        '''
        ## 解题思路：
        吃n个橘子所用天数f(n)有一下几种：
        1. 先吃1个，后面吃法有f(n-1), 总共吃法：1+f(n-1)
        2. n为2的倍数, 先吃n/2, 后面的f(n/2) ， 总共是1+f(n/2)
        3. n为3的倍数，先吃2n/3, 后面的(n/3)， 总共是1+f(n/3)
        最少天数为:
            f(n) = min(f(n-1), n%2+f(n/2), n%3+f(n/3)) + 1
        '''
        record = {}
        def eat(n: int) -> int:
            if n < 2:
                return n
            if n in record:
                return record.get(n)
            else:
                d = min(eat(n//2) + n%2, eat(n//3) + n%3) + 1
                record[n] = d
                return d

        return eat(n)


# @lc code=end

if __name__ == "__main__":
    s = Solution()
    print(s.minDays(10))

