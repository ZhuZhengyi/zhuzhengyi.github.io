/*
 * @lc app=leetcode.cn id=260 lang=cpp
 *
 * [260] 只出现一次的数字 III
 *
 * https://leetcode.cn/problems/single-number-iii/description/
 *
 * algorithms
 * Medium (73.51%)
 * Likes:    620
 * Dislikes: 0
 * Total Accepted:    92.5K
 * Total Submissions: 125.8K
 * Testcase Example:  '[1,2,1,3,2,5]'
 *
 * 给定一个整数数组 nums，其中恰好有两个元素只出现一次，其余所有元素均出现两次。 找出只出现一次的那两个元素。你可以按 任意顺序 返回答案。
 * 
 * 
 * 
 * 进阶：你的算法应该具有线性时间复杂度。你能否仅使用常数空间复杂度来实现？
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,2,1,3,2,5]
 * 输出：[3,5]
 * 解释：[5, 3] 也是有效的答案。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [-1,0]
 * 输出：[-1,0]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：nums = [0,1]
 * 输出：[1,0]
 * 
 * 
 * 提示：
 * 
 * 
 * 2 
 * -2^31 
 * 除两个只出现一次的整数外，nums 中的其他数字都出现两次
 * 
 * 
 */

// @lc code=start
class Solution {
public:
    /*
    * ## 解题思路
    * 1. 先将所有数字进行异或操作，得到一个异或结果xor, 由于存在两个不同的元素，则xor必不为0；
    * 2. 根据xor，计算mask = xor & -xor, 得到xor第一个1
    * 3. 使用mask与每个数字进行与操作，将nums分成两个子nums1， nums2，
    * 4. 分别对两个nums1, nums2执行xor操作，得到最后两个不同的数；
    */
    vector<int> singleNumber(vector<int>& nums) {
        long div = 0;
        int a = 0, b = 0;
        for(auto n: nums)
            div ^= n;
        div &= -div;
        
        for(auto n: nums){   
            if(div & n)
                a ^= n;
            else
                b ^= n;
        }
        return {a,b};
    }
};
// @lc code=end

