// greedy ac
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let n = nums.len();
        let mut count = 0;
        for i in 0..n{
            count += nums[i];
            if count>res{
                res = count;
            }
            if count<=0{
                count=0;
            }
        }
        return res;
    }
}
// 暴力 time out error
// impl Solution {
//     pub fn max_sub_array(nums: Vec<i32>) -> i32 {
//         let mut res = nums[0];
//         let n = nums.len();
//         for i in 0..n{
//             let mut count = 0;
//             for j in i..n{
//                 count += nums[j];
//                 if count > res{
//                     res = count;
//                 }
//             }
//         }
//         return res;
//     }
// }