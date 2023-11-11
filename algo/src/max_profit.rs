impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        // 前闭后开结构，类似于range(0,len-1)
        for i in 0..prices.len()-1{
            if prices[i+1]-prices[i]>0{
                res+=prices[i+1]-prices[i];
            }
        }
        return res;
    }
}


// py code
// class Solution:
//     def maxProfit(self, prices: List[int]) -> int:
//         delta = []
//         for i in range(len(prices)-1):
//             delta.append(prices[i+1]-prices[i])
//         res = 0
//         for i in delta:
//             if i > 0:
//                 res+=i
//         return res