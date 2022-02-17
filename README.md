# interview-note

## 目前状态

目前正在找 C++ Linux 后端开发的实习，前途未卜中。

[简历（未完成） Resume](./src/resume_v1.pdf) (网页无法正常显示，需下载)

## 面试记录

None

## 实习情况

None

## 刷题笔记

1. 数据结构
    - 栈
      - 栈/递归
      - 单调栈
    - 队列
    - 链表
    - 哈希表
    - 数组

2. 算法
    - 搜索
      - BFS
      - DFS
      - 回溯
    - 动态规划
    - 贪心思想
    - 二分思想
    - 双指针
    - 分而治之
    - 数学问题

3. 数据结构
    - 栈
      - 栈/递归
        - 水题：LC20-Easy、LC225-Easy、LC232-Easy
        - LC155-Easy 运用辅助栈,多使用一个栈来保存别的信息，让辅助栈来跟随主栈push\pop操作，并保存每一层的最小值情况。
        - LC394-Medium 解析上一层需要前需要先解析下一层得到信息，明显就是递归，将 [ 和 ] 分别作为递归的开启与终止条件。
      - 单调栈
    - 队列
      - 链表
        - 水题：LC21-Easy、LC83-Easy、LC206-Easy(用三个节点来实现反转链表)、LC-Easy、LC-Easy、LC-Easy
        - LC19-Medium 关于链表倒数第n个节点，先让一个指针指向正向第n个，然后另一个指针指向头结点，当第一个指针到末尾时，第二个指针就到了倒数第n个节点上。
        - LC160-Easy 找两个链表相交点，不适用额外空间的方法，就是利用 a+b+c = c+b+a，其中b是相交之后共同的部分，a和c分别是两个链表相交之前的部分，利用这个数学原理得到，走完a+b+c后，下一个一定是b的第一个，也就是相交的第一个节点；另外如果两个链表不相交，那么就是a+c=c+a，下一个都是nullptr，虽然不相交但也可以通过 l1 == l2 来终止循环,无需额外考虑。
        - LC234-Easy 不适用额外空间的做法，快慢指针找中间节点，然后后半个链表再反转（LC206），然后比较这两个链表。
        - LC445-Medium 这道题本身很简单，难点在于得到逆序的数值，关于逆序我们首先就要想到栈，使用栈很简单，比反转链表方便多了。
      - 哈希表
      - 数组

4. 算法
    - 搜索
      - BFS/DFS
        - LC93-Medium
        - LC130-Medium       
        - LC200-Medium
        - LC207-Medium
        - LC257-Easy
        - LC399-Medium
        - LC417-Medium
        - LC547-Medium
        - LC695-Easy
      - 回溯
    - 动态规划
    - 贪心思想
    - 二分思想
    - 双指针
    - 分而治之
    - 数学问题

## 其他笔记

None
