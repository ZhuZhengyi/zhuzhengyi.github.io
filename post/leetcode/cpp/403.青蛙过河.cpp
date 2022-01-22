/*
 * @lc app=leetcode.cn id=403 lang=cpp
 *
 * [403] 青蛙过河
 *
 * https://leetcode-cn.com/problems/frog-jump/description/
 *
 * algorithms
 * Hard (45.55%)
 * Likes:    396
 * Dislikes: 0
 * Total Accepted:    48.3K
 * Total Submissions: 106K
 * Testcase Example:  '[0,1,3,5,6,8,12,17]'
 *
 * 一只青蛙想要过河。 假定河流被等分为若干个单元格，并且在每一个单元格内都有可能放有一块石子（也有可能没有）。 青蛙可以跳上石子，但是不可以跳入水中。
 * 
 * 给你石子的位置列表 stones（用单元格序号 升序 表示）， 请判定青蛙能否成功过河（即能否在最后一步跳至最后一块石子上）。
 * 
 * 开始时， 青蛙默认已站在第一块石子上，并可以假定它第一步只能跳跃一个单位（即只能从单元格 1 跳至单元格 2 ）。
 * 
 * 如果青蛙上一步跳跃了 k 个单位，那么它接下来的跳跃距离只能选择为 k - 1、k 或 k + 1 个单位。
 * 另请注意，青蛙只能向前方（终点的方向）跳跃。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：stones = [0,1,3,5,6,8,12,17]
 * 输出：true
 * 解释：青蛙可以成功过河，按照如下方案跳跃：跳 1 个单位到第 2 块石子, 然后跳 2 个单位到第 3 块石子, 接着 跳 2 个单位到第 4 块石子,
 * 然后跳 3 个单位到第 6 块石子, 跳 4 个单位到第 7 块石子, 最后，跳 5 个单位到第 8 个石子（即最后一块石子）。
 * 
 * 示例 2：
 * 
 * 
 * 输入：stones = [0,1,2,3,4,8,9,11]
 * 输出：false
 * 解释：这是因为第 5 和第 6 个石子之间的间距太大，没有可选的方案供青蛙跳跃过去。
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 2 
 * 0 
 * stones[0] == 0
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /*
    ## 解题思路
    * * 动态规划
    *   令 dp[i][k]: 跳到第i个stone，上一步跳跃k个单位可达情况，则有：
    *       dp[i][k] = dp[j][k-1] || dp[j][k] || dp[j][k+1]
    *   其中j, i 满足：
    *       stones[i] - stones[j] = k
    * 
    * * 初始条件：
    *       dp[0][0] = true   //最开始站在stones[0]上面
    * 
    * * 目标值： 
    *       dp[n-1][k]   //到达stones[n-1]
    * 
    * * 剪支条件：
    *   1. 第i块石头，最远可达距离为stones[i] + i+1 (跳到i的最大k为i) ，因此
    *       stones[i] - stones[i-1] > i 
    *      时，第i块石头必定无法到达
    *   2. 
    */
    bool canCross(vector<int>& stones) {
        int n = stones.size();
        vector<vector<bool>> dp(n, vector<bool>(n, false));
        dp[0][0] = true;

        for(int i=1; i<n; i++) {
            // 第i-1块石头的最大可跳范围为i(从0开始跳i-1步到i-1, 此时第i-1步可跳的范围最大，为i-1+1=i)
            // 若第i块石头的距离stones[i] > 第i-1的距离stones[i-1] + i, 则无法到达第i块石头
            if (stones[i] > stones[i-1] + i) {
                return false;
            } 
            for(int j=i-1; j>=0; j--) {
                int k = stones[i] - stones[j];
                if (j<k-1) break;   //j如果小于k-1, 则无法跳到i;
                dp[i][k] = dp[j][k-1] || dp[j][k] || dp[j][k+1];
                if(i==n-1&&dp[i][k]) {
                    return true;
                }
            }
        }

        return false;
    }
};
// @lc code=end

