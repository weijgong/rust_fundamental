# 通过魔改力扣最长连续序列长度
# 将连续的定义修改为：1.大于5，保证不为接近0的数值；2.两个数之间的差值绝对值不超过2。
def judge(pre,back):
    if pre < 5 or back <5:
        return False
    if abs(pre - back)>2:
        return False
    return True

# 用于实现需求功能的小破遍历+排序
class Solution:
    def longestConsecutive(self, nums) -> int:
        n = len(nums)
        if n<=1:
            return n
        nums.sort()
        max_l = 1
        tmp = 1
        for i in range(n-1):
            di = nums[i+1]-nums[i]
            if di in [0,1]:
                tmp += di
            else:                
                tmp=1
            max_l = max(max_l,tmp)
        return max_l
            
