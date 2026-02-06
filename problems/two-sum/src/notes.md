# Two Sum Algorithm Explanation

**Problem Approach**

The solution uses a simple but efficient approach:

For each number in the array, we calculate the needed complement as:

```
needed_number = target - current_number
```

**Key Insight**

- If `current_number + needed_number = target`, we've found a pair that sums to the target
- We can check if we've already seen the needed_number earlier in the array

**Data Structure Choice**

- **HashMap**: Provides O(1) time complexity for lookups
- **Tradeoff**: Space complexity is O(n) as the map can grow to store all elements

**Algorithm Steps**

1. Iterate through the array with `enumerate()` to track indices
2. For each element:
   - Calculate `needed_number = target - current_number`
   - Check if `needed_number` exists in the HashMap
     - **If exists**: Return both indices (value from map + current index)
     - **If not exists**: Insert `(current_number, current_index)` into the HashMap

3. Cast both indices to `i32` and return in a vector

**Complexity Analysis**

- **Time Complexity**: O(n) - single pass through the array
- **Space Complexity**: O(n) - HashMap storage
