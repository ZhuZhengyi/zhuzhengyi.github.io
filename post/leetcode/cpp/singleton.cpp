/*
@Copyright:LintCode
@Author:   justice_103
@Problem:  http://www.lintcode.com/problem/singleton
@Language: C++
@Datetime: 15-08-29 22:59
*/

class Solution {
public:
    /**
     * @return: The same instance of this class every time
     */
    static Solution* getInstance() {
        // write your code here
        static Solution instance;
        return &instance;
    }
    
};
