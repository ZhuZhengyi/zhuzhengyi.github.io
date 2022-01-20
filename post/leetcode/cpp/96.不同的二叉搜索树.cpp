/*
 * @lc app=leetcode.cn id=96 lang=cpp
 *
 * [96] 不同的二叉搜索树
 *
 * https://leetcode-cn.com/problems/unique-binary-search-trees/description/
 *
 * algorithms
 * Medium (69.94%)
 * Likes:    1502
 * Dislikes: 0
 * Total Accepted:    192K
 * Total Submissions: 274.5K
 * Testcase Example:  '3'
 *
 * 给你一个整数 n ，求恰由 n 个节点组成且节点值从 1 到 n 互不相同的 二叉搜索树 有多少种？返回满足题意的二叉搜索树的种数。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 3
 * 输出：5
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 1
 * 输出：1
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /*
    ## 解题思路
    * 动态规划
      G(n):  n个节点二叉排序树的个数
      f(i): 为以i为根的二叉搜索树的个数，
      则  
           G(n)=f(1)+f(2)+f(3)+f(4)+...+f(n)
      当i为根节点时，其左子树节点个数为 i-1 个，右子树节点为 n-i，则
           f(i) = G(i-1) * G(n-i)
       综合得到 卡特兰数 公式
           G(n) = G(0)*G(n-1)+G(1)*(n-2)+...+G(n-1)*G(0)

    */
    int numTrees(int n) {
        if (n==1) return 1;
        vector<int> g(n+1);
        g[0] = 1;
        g[1] = 1;
        for (int i=2; i<=n; ++i) {
            for (int j=1; j<=i; ++j) {
                g[i] += g[j-1]*g[i-j];
            }
        }

        return g[n];
    }
};
// @lc code=end

