我主要是把rust的一些教程顺着敲了一遍，一方面是体验一下rust编程过程中的基础知识，另一方面是便于后续学习rCore相关编程。学习的链接为：
> [例子学rust](https://rustwiki.org/zh-CN/rust-by-example/)
> [Rust中文手册](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)

这里稍微记录一些Python转Rust过程中的思路：
- 迭代相关：像是for i in range(10)在rust中为：for 1..11{}，或者for 1..=10{}，while的话则基本一致，只需要把while :转化为while{}
- 