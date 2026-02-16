fn main() {
    fn median_of_two_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() < nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };

        let m = a.len();
        let n = b.len();
        let total_len = m + n;
        let half_len = (m + n + 1) / 2;
        let mut left = 0;
        let mut right = m;
        while left <= right {
            let partition_a = (left + right) / 2;
            let partition_b = half_len - partition_a;

            let a_left = if partition_a > 0 {
                a[partition_a - 1]
            } else {
                i32::MIN
            };

            let a_right = if partition_a < m {
                a[partition_a]
            } else {
                i32::MAX
            };

            let b_left = if partition_b > 0 {
                b[partition_b - 1]
            } else {
                i32::MIN
            };

            let b_right = if partition_b < n {
                b[partition_b]
            } else {
                i32::MAX
            };

            if a_left <= b_right && b_left <= a_right {
                if total_len % 2 == 0 {
                    return (a_left.max(b_left) + a_right.min(b_right)) as f64 / 2 as f64;
                } else {
                    return a_left.max(b_left) as f64;
                }
            } else if a_left > b_right {
                right = partition_a - 1;
            } else {
                left = partition_a + 1;
            }
        }

        0.0
    }

    let arr1 = vec![1, 3];
    let arr2 = vec![2];

    let result = median_of_two_sorted_arrays(arr1, arr2);
    assert_eq!(2.0, result);
    println!("{}", result);
}
