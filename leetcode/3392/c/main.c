int countSubarrays(int* nums, int numsSize) {
    int cnt = 0;
    for (int i = 0; i < numsSize - 2; i++) {
        int mid = nums[i + 1] / 2;
        if (mid * 2 == nums[i + 1] && (nums[i] + nums[i + 2]) == mid) {
            cnt++;
        }
    }

    return cnt;
}
