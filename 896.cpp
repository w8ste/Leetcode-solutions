#include <stdio.h>
#include <stdbool.h>
#include <vector>

class Solution {
public:
  bool isMonotonic(std::vector<int>& nums) {
        bool inc = true;
        bool mode = false;

        for(int i = 0; i < nums.size() - 1; i++) {
            if(mode) {
                if((inc && nums[i] > nums[i + 1]) || (!inc && nums[i] < nums[i + 1])) return false;
            }
            else {
                if(nums[i] < nums[i + 1]) {
                    mode = true;
                }
                else if (nums[i] > nums[i + 1]) {
                    mode = true;
                    inc = false;
                }
            }
        } 
        return true;       
    }
};
