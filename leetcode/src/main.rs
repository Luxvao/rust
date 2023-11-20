struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged: Vec<i32> = Vec::new();

        for i in nums1.iter() {
            merged.push(*i);
        }

        for i in nums2.iter() {
            merged.push(*i);
        }

        let output = sort_vec(&merged);

        println!("{:?}", output);

        return 0.2;
    }
}

fn sort_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut running = true;
    let mut counter = 0;
    let mut output = vec.clone();

    while running {
        for (i, item) in vec.iter().enumerate() {
            if counter == vec.len() {
                running = false;
            }
            else if counter + 1 >= vec.len() {
                continue;
            }
            else if item > &vec[i + 1] {
                let tmp = item.clone();

                output[i] = vec[i + 1];
                output[i + 1] = tmp;
                counter = 0;
            }
            else {
                counter = counter + 1;
            }
        }
    }

    return output;
}

fn main() {
    Solution::find_median_sorted_arrays(vec![1, 3], vec![2]);
}
