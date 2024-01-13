#include <string>
#include <unordered_map>
 

using namespace std;
class Solution {
public:
  int minSteps(std::string s, string t) {
        int steps = 0;
        unordered_map<int, int> map;
        for (auto x : s) {
            map[x - 'a']++; 
        }
        for (auto x : t){
            map[x - 'a']--;
        } 
        for (auto x : map) if (x.second > 0) {
            steps += x.second;
        }
        return steps;
    }
};
