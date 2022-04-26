/*
 * @lc app=leetcode.cn id=46 lang=cpp
 *
 * [46] 全排列
 *
 * https://leetcode-cn.com/problems/permutations/description/
 *
 * algorithms
 * Medium (78.51%)
 * Likes:    1963
 * Dislikes: 0
 * Total Accepted:    595K
 * Total Submissions: 757.9K
 * Testcase Example:  '[1,2,3]'
 *
 * 给定一个不含重复数字的数组 nums ，返回其 所有可能的全排列 。你可以 按任意顺序 返回答案。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,2,3]
 * 输出：[[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [0,1]
 * 输出：[[0,1],[1,0]]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：nums = [1]
 * 输出：[[1]]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= nums.length <= 6
 * -10 <= nums[i] <= 10
 * nums 中的所有整数 互不相同
 * 
 * 
 */
#include<vector>
#include<set>

using namespace std;

// @lc code=start
class Solution {
    vector<vector<int>> result;
public:
    /*
    * ## 解题思路
    * * 回溯法
    * 1. 依次从nums中取出一个元素，加入到tmp数组尾部；
    * 2. 将取出元素的下标保存在visited 集合中，用于标记已取出的元素；
    * 3. 取出一个后，递归取出其他未取出的元素；
    * 4. 当tmp的个数满了后，递归函数返回，将tmp加入到结果列表中；
    * 5. 递归函数返回时，需将之前所取元素放回，再处理下一个元素；
    */
    vector<vector<int>> permute(vector<int>& nums) {
        vector<int> tmp;            //一次排列
        set<int> visited;           //记录已取元素下标
        dfs(nums, tmp, visited);

        return result;
    }

    void dfs(vector<int>&nums, vector<int>& tmp, set<int>& visited) {
        if (tmp.size() == nums.size()) {
            result.push_back(tmp);
            return;
        }
        for(int j=0; j<nums.size(); j++) {
            //跳过已取出的元素
            if (visited.find(j)!=visited.end()) {
                continue;
            }
            //将未取出的元素取出，加入到tmp中
            tmp.push_back(nums[j]);
            visited.insert(j);  //并记录到visited 集合中；
            //递归处理其他元素
            dfs(nums, tmp, visited);
            //递归返回时，放回已取出的元素
            tmp.pop_back();
            visited.erase(j);
        }
    }
};
// @lc code=end

int main() {
    Solution s;
    vector<int> nums = {1,2,3};
    printf(s.permute(nums));

    return 0;
}
