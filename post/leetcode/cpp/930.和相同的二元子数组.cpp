/*
 * @lc app=leetcode.cn id=930 lang=cpp
 *
 * [930] 和相同的二元子数组
 *
 * https://leetcode-cn.com/problems/binary-subarrays-with-sum/description/
 *
 * algorithms
 * Medium (34.68%)
 * Likes:    41
 * Dislikes: 0
 * Total Accepted:    2.6K
 * Total Submissions: 7.4K
 * Testcase Example:  '[1,0,1,0,1]\n2'
 *
 * 在由若干 0 和 1  组成的数组 A 中，有多少个和为 S 的非空子数组。
 * 
 * 
 * 
 * 示例：
 * 
 * 输入：A = [1,0,1,0,1], S = 2
 * 输出：4
 * 解释：
 * 如下面黑体所示，有 4 个满足题目要求的子数组：
 * [1,0,1,0,1]
 * [1,0,1,0,1]
 * [1,0,1,0,1]
 * [1,0,1,0,1]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * A.length <= 30000
 * 0 <= S <= A.length
 * A[i] 为 0 或 1
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    int numSubarraysWithSum(vector<int>& A, int S) {
        int len = A.size();
        int res = 0;
        int j = 0;
        int s = 0;
        for (int i = 0;  i < len; i++) {
            while( s <= S && j < len) {
                s += A[j];
                j++;
                if (s == S) {
                    res++;
                }
            }
            //j--;
        }

        return res;
    }
};
// @lc code=end

