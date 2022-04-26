/*
 * @lc app=leetcode.cn id=17 lang=cpp
 *
 * [17] 电话号码的字母组合
 *
 * https://leetcode-cn.com/problems/letter-combinations-of-a-phone-number/description/
 *
 * algorithms
 * Medium (57.75%)
 * Likes:    1863
 * Dislikes: 0
 * Total Accepted:    469.9K
 * Total Submissions: 813.7K
 * Testcase Example:  '"23"'
 *
 * 给定一个仅包含数字 2-9 的字符串，返回所有它能表示的字母组合。答案可以按 任意顺序 返回。
 * 
 * 给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。
 * 
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：digits = "23"
 * 输出：["ad","ae","af","bd","be","bf","cd","ce","cf"]
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：digits = ""
 * 输出：[]
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：digits = "2"
 * 输出：["a","b","c"]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= digits.length <= 4
 * digits[i] 是范围 ['2', '9'] 的一个数字。
 * 
 * 
 */

#include<vector>
#include<string>
#include<unordered_map>

using namespace std;


// @lc code=start
class Solution {
	vector<string> result;
	vector<string> letters = {"abc","def","ghi","jkl","mno","pqrs","tuv","wxyz"};
public:
	/*
	* ## 解题思路
	* * 回溯法
	* 1. 依次尝试每个数字对应的各个字母
	*/
	vector<string> letterCombinations(string digits) {
		if (digits.empty()) {
			return result;
		}
		string path = "";
		dfs(digits, path, 0);
		return result;
	}

	//
	void dfs(string& digits, string& path, int i) {
		if (i>=digits.length()){
			result.push_back(path);
			return;
		}

		for(char c: letters[digits[i]-'2']) {
			path.push_back(c);
			dfs(digits, path, i+1);
			path.pop_back();
		}
	}
};
// @lc code=end

