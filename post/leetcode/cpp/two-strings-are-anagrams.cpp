/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/two-strings-are-anagrams
@Language: C++
@Datetime: 15-07-07 12:14
*/

class Solution {
public:
    /**
     * @param s: The first string
     * @param b: The second string
     * @return true or false
     */
    bool anagram(string s, string t) {
        // write your code here
        for( auto c : s )
            if( t.find(c) == string::npos )
                return false;
        return true;
    }
};
