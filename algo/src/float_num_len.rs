impl float_num_len{
    pub fn wiggle_max_length(nums: Vec<i32>)->i32{
        if nums.len() == 1{
            return 1;
        }
        let mut res = 1;
        let mut pre_diff = 0;
        let mut cur_diff = 0;
        for i in 0..nums.len-1{
            cur_diff = nums[i+1] - nums[i];
            if (pre_diff>=0 && cur_diff<0) || (pre_diff<=0&&cur_diff>0){
                res+=1;
                pre_diff = cur_diff;
            }
        }
    }
}