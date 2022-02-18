# interview-note

目前正在找 C++ Linux 后端开发的实习，前途未卜中。

[简历（未完成） Resume](./src/resume_v1.pdf) (网页无法正常显示，需下载)

## 面试记录

None

---

## 实习情况

None

---

## 应试笔记

1. [C++](#C++)
2. [数据结构/算法](#数据结构算法)

### C++

None

### 数据结构/算法

1. 数据结构
    - [栈](#栈)
      - [栈/递归](#栈递归)
      - [单调栈](#单调栈)
    - [链表](#链表)

2. 算法
    - [搜索](#搜索)
      - [BFS/DFS](#BFSDFS)
      - 回溯
    - 动态规划
    - 贪心思想
    - 二分思想
    - 双指针
    - 分而治之
    - 数学问题
    - LRU

#### 栈

##### 栈/递归

水题：LC20 Easy、LC225 Easy、LC232 Easy

|题号|笔记|
|:-|:-|
|LC155 Easy|运用辅助栈,多使用一个栈来保存别的信息，让辅助栈来跟随主栈push\pop操作，并保存每一层的最小值情况。|
|LC394 Medium|解析上一层需要前需要先解析下一层得到信息，明显就是递归，将 [ 和 ] 分别作为递归的开启与终止条件。|

##### 单调栈

#### 链表
   
水题：LC2 Medium、LC21 Easy、LC24 Medium、LC83 Easy、LC206 Easy、LC328 Medium、LC725 Medium

|题号|笔记|
|:-|:-|
|LC19 Medium|链表倒数第n个节点，先让一个指针指向正向第n个，然后另一个指针指向头结点，当第一个指针到末尾时，第二个指针就到了倒数第n个节点上。|
|LC109 Medium|使用快慢指针找链表中间节点。|
|LC141 Easy| 环形链表，判断是否有环，使用快慢指针可以保证空间复杂度为O(1)，因为两者速度差一，使得当两个指针都进入环内的初始位置后，每一步都会使得两指针距离减一，推得如果有环那么肯定会相交。|
|LC142 Medium|环形链表，要找到环开始的节点，快慢指针很麻烦，使用哈希表很简单，两者时间复杂度都是O(n)，不过哈希表空间复杂度是O(n)；如果要使用快慢指针，推出a=c+(n−1)(b+c)后，让新的指针从头走，slow指针接着走，发现当新指针走了a步，slow指针走了c步和n-1圈，刚好相交，这就是起始点，所以我们直接找新指针和slow指针第一次相交即可，此时就是起始点（https://leetcode-cn.com/problems/linked-list-cycle-ii/solution/huan-xing-lian-biao-ii-by-leetcode-solution/）。|
|LC160 Easy|两个链表相交点，不使用额外空间的方法，就是利用 a+b+c = c+b+a，其中b是相交之后共同的部分，a和c分别是两个链表相交之前的部分，利用这个数学原理得到，走完a+b+c后，下一个一定是b的第一个，也就是相交的第一个节点；另外如果两个链表不相交，那么就是a+c=c+a，下一个都是nullptr，虽然不相交但也可以通过 l1 == l2 来终止循环,无需额外考虑。|
|LC234 Easy|不使用额外空间的做法，快慢指针找中间节点，然后后半个链表再反转（使用前中后三个节点来实现反转），然后比较这两个链表。
|LC445 Medium|这道题本身很简单，难点在于得到逆序的数值，关于逆序我们首先就要想到栈，使用栈很简单，比反转链表方便多了。|

#### 数组

水题：LC108 Easy

#### 搜索

##### BFS/DFS

水题：

|题号|笔记|
|:-|:-|
|LC93 Medium||
|LC130 Medium||
|LC200 Medium||
|LC207 Medium||
|LC257 Easy||
|LC399 Medium||
|LC417 Medium||
|LC547 Medium||
|LC695 Easy||

### to do
