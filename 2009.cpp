#include <algorithm>
#include <stdio.h>
#include <vector>

using namespace std;
class Solution {
public:
  Solution(const Solution &) = default;
  Solution(Solution &&) = default;
  Solution &operator=(const Solution &) = default;
  Solution &operator=(Solution &&) = default;
  int minOperations(vector<int> &nums) {
    sort(nums.begin(), nums.end());
    vector<int> sec;
    sec.push_back(nums[0]);
    for (int i = 1, j = 0; i < nums.size(); i++) {
      if (nums[i] != sec[j]) {
        sec.push_back(nums[i]);
        j++;
      }
    }
    int operations = nums.size() - 1;
    for (int i = 0, j = 0; i < sec.size(); i++) {
      while (j < sec.size() && sec[j] < sec[i] + nums.size()) {
        j++;
      }
      if (nums.size() - j + i < operations) {
        operations = nums.size() - j + i;
      }
    }
    return operations;
  }
};
