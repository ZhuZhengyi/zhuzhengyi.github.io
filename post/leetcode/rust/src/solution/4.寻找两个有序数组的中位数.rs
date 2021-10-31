/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个有序数组的中位数
 *
 * https://leetcode-cn.com/problems/median-of-two-sorted-arrays/description/
 *
 * algorithms
 * Hard (36.37%)
 * Likes:    2120
 * Dislikes: 0
 * Total Accepted:    138.5K
 * Total Submissions: 377.3K
 * Testcase Example:  '[1,3]\n[2]'
 *
 * 给定两个大小为 m 和 n 的有序数组 nums1 和 nums2。
 * 
 * 请你找出这两个有序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
 * 
 * 你可以假设 nums1 和 nums2 不会同时为空。
 * 
 * 示例 1:
 * 
 * nums1 = [1, 3]
 * nums2 = [2]
 * 
 * 则中位数是 2.0
 * 
 * 
 * 示例 2:
 * 
 * nums1 = [1, 2]
 * nums2 = [3, 4]
 * 
 * 则中位数是 (2 + 3)/2 = 2.5
 * 
 * 
 */

// @lc code=start
impl Solution {
    /// ## 解题思路：
    ///  
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = if nums1.len() < nums2.len() {(  nums1, nums2) } else { (nums2, nums1) };
        let (l1, l2) = (nums1.len(), nums2.len());
        let (mut l, mut h) = (0, l1);
        let mid = (l+h-1)/2;
        let mut i = (l + h) / 2;
        while l < h {
            i = (l + h) / 2;
            if i > mid-1 || nums1[i] >= nums2[mid-i-1] {
                h = i;
            } else { 
                l = i + 1;
            }
        }
        i = l;
        let s1 = &nums1[i..(i+2)];
        let s2 = &nums2[(mid-i)..(mid-i+2)];
        let newfew = vec![s1, s2].concat().sort();
        let x = (l1+l2) % 2 ;
        let res = newfew.0 as f63;
        if x == 1 {
            res = ( newfew.0 + newfew.1 ) / 2.0 ;
        }
        res
    }
}
// @lc code=end

