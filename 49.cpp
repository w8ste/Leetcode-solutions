class Solution {
public:
  vector<vector<string>> groupAnagrams(vector<string>& strs) {
        vector<vector<string>> sol;
        unordered_map <string, vector<string>> map;

        for(const string& s : strs) {
            string k = s;
            ranges::sort(k);
            map[k].push_back(s);
        }

        for (const auto& [_, anagrams] : map)
            sol.push_back(anagrams);

    return sol;

        
    }
};
