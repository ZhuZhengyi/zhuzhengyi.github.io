/*
 * @lc app=leetcode.cn id=33 lang=rust
 *
 * [33] 搜索旋转排序数组
 *
 * https://leetcode.cn/problems/search-in-rotated-sorted-array/description/
 *
 * algorithms
 * Medium (43.81%)
 * Likes:    2570
 * Dislikes: 0
 * Total Accepted:    714.1K
 * Total Submissions: 1.6M
 * Testcase Example:  '[4,5,6,7,0,1,2]\n0'
 *
 * 整数数组 nums 按升序排列，数组中的值 互不相同 。
 *
 * 在传递给函数之前，nums 在预先未知的某个下标 k（0 <= k < nums.length）上进行了 旋转，使数组变为 [nums[k],
 * nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]（下标 从 0 开始
 * 计数）。例如， [0,1,2,4,5,6,7] 在下标 3 处经旋转后可能变为 [4,5,6,7,0,1,2] 。
 *
 * 给你 旋转后 的数组 nums 和一个整数 target ，如果 nums 中存在这个目标值 target ，则返回它的下标，否则返回 -1 。
 *
 * 你必须设计一个时间复杂度为 O(log n) 的算法解决此问题。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [4,5,6,7,0,1,2], target = 0
 * 输出：4
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [4,5,6,7,0,1,2], target = 3
 * 输出：-1
 *
 * 示例 3：
 *
 *
 * 输入：nums = [1], target = 0
 * 输出：-1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= nums.length <= 5000
 * -10^4 <= nums[i] <= 10^4
 * nums 中的每个值都 独一无二
 * 题目数据保证 nums 在预先未知的某个下标上进行了旋转
 * -10^4 <= target <= 10^4
 *
 *
 */

// @lc code=start
impl Solution {
    /// ## 解题思路
    /// - 二分查找
    /// 1. 旋转数组的旋转点(nums[i] < nums[i-1]的点)将数组分为左右两个有序部分;
    /// 2. 根据左右两部分的长度不同, 分两种情况分别处理;
    /// 3. 当nums[mid]>nums[l]时, 表明左边的比较长, 否则右边比较长;
    /// 4. nums[mid]和target比较时, 根据左右长度的不同,分情况判断丢弃的部分;
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        if nums.len() == 1 {
            if nums[0] == target { 
                return 0;
            } else {
                -1;
            }
        }
        while l < r {
            if nums[l] == target {
                return l as i32;
            }
            if nums[r] == target {
                return r as i32;
            }
            let mid = (l + r + 1) / 2; 
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] > nums[l] { //左边长, 右边短
                if nums[mid] > target && target > nums[l] { //中间值>target
                    r = mid - 1;
                } else {   
                    l = mid + 1;
                }
            } else { //左边短,右边长
                if nums[mid] < target && target < nums[l] { //中间值<target
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
        }

        -1
    }
}
// @lc code=end

struct Solution;
