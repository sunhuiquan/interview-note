# interview-note

目前正在找 C++ Linux 后端开发的实习，前途未卜中。

[简历 Resume](./src/resume_v2.pdf) (网页无法正常显示，需下载)

## 面试记录

None

## 实习情况

None

## 应试笔记

1. [C++](#C)
2. [数据结构/算法](#数据结构算法)

### **C++**

None

### **数据结构/算法**

1. 数据结构
    - [数组](#数组)
    - [栈](#栈)
      - [栈/递归](#栈递归)
      - [单调栈](#单调栈) 739 496 503 769
    - [队列](#队列)
      - [普通队列](#普通队列)
      - [堆(优先队列)](#堆优先队列)
    - [链表](#链表)
    - [字符串](#字符串)
      - [常规字符串](#常规字符串)
      - [大数问题](#大数问题)
    - [哈希表](#哈希表)
    - [树](#树)
      - [普通题型](#普通题型)
      - [遍历](#遍历)
      - [BST(二叉搜索树)](#BST(二叉搜索树))
      - [Trie(字典树)](#Trie(字典树))
    - [图](#图)
    - [位运算](#位运算)

2. 算法
    - [搜索](#搜索)
      - [BFS/DFS](#BFSDFS)
      - [回溯](#回溯) 301 526
    - [排序](#排序)
      - [简单的排序](#简单的排序)
      - [快速选择](#快速选择)
      - [堆排序](#堆排序)      
      - [桶排序](#桶排序)
      - [归并排序](#归并排序)
    	<!-- 排序 https://leetcode-cn.com/problems/sort-an-array/solution/pai-xu-shu-zu-by-leetcode-solution/
    	快速选择
    	 -->
    - [双指针](#双指针) 167 345 633 680 88 524 \ 11 15 31 42 75 148 253 581
    - [动态规划](#动态规划) 3 5 10 22 32 53 72 121 139 338 647
		<!-- 斐波那契数列
		最长递增子序列	
		最长公共子序列
		0-1 背包
		数组区间
		分割整数
		矩阵路径
		其它问题 -->
    - [贪心思想](#贪心思想)
    - [二分思想](#二分思想)
    - [分而治之](#分而治之)
    - [数学问题](#数学问题)
		<!-- 素数（埃氏筛 204）
		最大公约数
		相遇问题
		多数投票问题
		其它 -->
	  - [进制转换](#进制转换)
	  - [阶乘](#阶乘)
	  - [相遇问题](#相遇问题)
    - [LRU](#LRU)
    - [滑动窗口](#滑动窗口) 3 76 219 239 438

### **数组**

水题：LC108 Easy、LC240 Medium、LC448 Easy、LC485 Easy、LC566 Easy、LC654 Medium、LC697 Easy、LC766 Easy

|题号|笔记|
|:-|:-|
|LC283 Easy|双指针交换类型，注意当左索引没有指0的时候，左右索引相同，交换的是自身。|
|LC287 Medium||
|LC442 Medium||

### **栈**

#### 栈/递归

水题：LC20 Easy、LC225 Easy、LC232 Easy

|题号|笔记|
|:-|:-|
|LC155 Easy|运用辅助栈,多使用一个栈来保存别的信息，让辅助栈来跟随主栈push\pop操作，并保存每一层的最小值情况。|
|LC394 Medium|解析上一层需要前需要先解析下一层得到信息，明显就是递归，将 [ 和 ] 分别作为递归的开启与终止条件。|

#### 单调栈

### **队列**

#### 普通队列

水题：LC281 Medium

#### 堆(优先队列)

跳转：[堆排序](#堆排序)

### **链表**
   
水题：LC2 Medium、LC21 Easy、LC24 Medium、LC83 Easy、LC206 Easy、LC328 Medium、LC725 Medium

|题号|笔记|
|:-|:-|
|LC160 Easy|两个链表相交点，不使用额外空间的方法，就是利用 a+b+c = c+b+a，其中b是相交之后共同的部分，a和c分别是两个链表相交之前的部分，利用这个数学原理得到，走完a+b+c后，下一个一定是b的第一个，也就是相交的第一个节点；另外如果两个链表不相交，那么就是a+c=c+a，下一个都是nullptr，虽然不相交但也可以通过 l1 == l2 来终止循环,无需额外考虑。|
|LC234 Easy|不使用额外空间的做法，快慢指针找中间节点，然后后半个链表再反转（使用前中后三个节点来实现反转），然后比较这两个链表。
|LC445 Medium|这道题本身很简单，难点在于得到逆序的数值，关于逆序我们首先就要想到栈，使用栈很简单，比反转链表方便多了。|

### **字符串**

#### 常规字符串

水题：LC9 Easy、LC205 Easy、LC242 Easy、LC409 Easy

|题号|笔记|
|:-|:-|
|LC49 Medium|注意包含相同字母(个数也相同)的单词，可以经过排序，得到一个相同的单词作为key，然后使用哈希。|
|LC696 Easy|一些时候可以可以将同字符的子字符串转为单纯的一个数字，然后利用数学关系简化复杂度。|

#### 大数问题

|题号|笔记|
|:-|:-|
|LC43 Medium|大数乘法，直接模拟的话，就是一轮一轮乘，然后把每一轮的结果再通过大数相加得到最终答案，复杂度(mn + n^2),复杂度和O(mn)或者O(n^2)差不多|
|LC415 Easy|从低位到高位模拟运算即可，注意字符串加的顺序，str = 新的高位 + str，再就是注意下进位。|

### **哈希表**

### **树**

#### 普通题型

水题：LC112 Easy、LC404 Easy

|题号|笔记|
|:-|:-|
|LC101 Easy|对称，让根节点的左右节点，如此check(p->left, q->right) && check(p->right, q->left)递归比较；或者是先反转任一一个子树，然后递归比较相同。|
|LC104 Easy|求树的高度。|
|LC111 Easy|求树的最小深度，注意不同于普通的求高度，当叶节点情况返回1，当本身是nullptr说明这里无该节点，这里的深度无意义，返回一个INT32_MAX舍去这种无意义情况。|
|LC110 Easy|自顶而下递归比较左右两子树的高度很简单，但是内部有重复，复杂度是O(n^2)；与自定而下相对应的就是自底向上，复杂度仅O(n)，思路参见该表格中下面的LC543题的笔记解释。|
|LC226 Easy|反转二叉树。|
to do另一种方法|LC437 Medium|路径求和，可以遍历并对每个节点来一次递归求解操作，复杂度O(n^2)，写法简单；另一种利用前缀和，降低复杂度只有O(n)。|
|LC235 Easy|BST下找最近公共父结点，分类讨论，利用分叉点一定最近来只需要O(n)的复杂度；另一种做法是分别保存根节点到p、q的两个路径，然后比较得到。|
|LC236 Medium||
|LC543 Easy|求树的直径，转换成对每个节点求左子树和右子树的高度和，返回最大的即可。简单的做法是双重递归，对每个节点都递归求解；但是更高效的一种方法，我们发现求深度的递归函数中，内部会获取左子树和右子树的高度，利用这里可以可以求最大直径，而这里的方向是从下往上，因为是深度遍历，最开始得到左右子树两个深度的情况是最底层，然后再return返回到上一层继续比较，这就是自底向上的思想。|
to do另一种方法|LC572 Easy|利用一个性质，那就是相同树结构的先序遍历结果相同（相同的结果一定相同树吗？？）|
|LC617 Easy|合并二叉树。|
|LC669 Medium|注意处理根节点的情况，发现根节点不满足问题就变成求它的一个子树的范围。（迭代写了三十行，递归只要十行无语。。）|
|LC671 Easy||

#### 遍历

水题：LC94 Easy、LC144 Easy、LC145 Easy、LC513 Medium

|题号|笔记|
|:-|:-|
|LC637 Easy|层次遍历，外层while条件是队列不为空，内部一个for循环是当前层的结点遍历，for循环的次数是通过队列size函数来得到的，即当前层结点数。|
|||
|||
|||
|||

#### BST(二叉搜索树)

|题号|笔记|
|:-|:-|
|LC230|BST按中序遍历会得到一个递增的序列，中序遍历+中间计数即可求得BST中第k小的元素了。因为中序遍历有个特性是先到最左面的叶子结点，那么复杂度是O(H+k)，H是树的高度，所以如果我们将BST优化成AVL能把复杂度最小化成O(log N + k)，因为AVL树的高度是log N。|
|LC501|利用BST中序遍历是非递减序列完成，注意通过非递减这个特性，我们可以不用哈希表，从而空间复杂度为O(1)。|
|LC530 Easy|依旧是利用BST最重要的那个性质，中序遍历得到非递减序列，依次做差即可。|
|LC538 Medium|这道题使用了逆中序遍历的思路，先右再左遍历。既然中序遍历是递增，那么逆中序自然是递减，方向是从右往左，累加和也变得可以计算了。|
|LC653 Easy|用烂了的BST中序遍历+双指针即可。|

#### Trie(字典树)

|题号|笔记|
|:-|:-|
|LC208 Medium|典型的字典树写法，另外注意下利用是否是叶节点的布尔变量来区分查前缀和搜索单词。|
|LC677 Medium|字典序前缀和的问题，注意可以优化每个节点保存一个前缀和的方式，通过每次新插入增加路径前缀节点的值；另外注意通过哈希表记录以及插入的值，发生改变值的时候，我们就这次新值和之前值做差，然后增加前缀节点这个差值，完成更改。|

### **图**

### **位运算**

水题：

|题号|笔记|
|:-|:-|
|||
|||
|||
|||
|||

### **搜索**

#### BFS/DFS

水题：

|题号|笔记|
|:-|:-|
|LC93 Medium||
|LC130 Medium||
|LC200 Medium|典型的二维方格问题，岛屿计数，遍历进行dfs，每一次遇到未访问的点加一计数即可。四个方向可以定义两个方向数组，写成循环减少代码量，而不用写四遍。|
|LC207 Medium||
|LC257 Easy|典型的树的递归遍历，注意通过参数来保存当前深度的已经过的路径，递归通过参数保存当前状态，和向下再传递状态。|
|LC386 Medium|按字典序用dfs遍历即可。|
|LC399 Medium||
|LC417 Medium||
|LC547 Medium||
|LC695 Easy||

#### 回溯

|题号|笔记|
|:-|:-|
17
79
46
47
77
39
40
216
78
90
131
37
51

### **双指针**

|题号|笔记|
|:-|:-|
|LC19 Medium|链表倒数第n个节点，先让一个指针指向正向第n个，然后另一个指针指向头结点，当第一个指针到末尾时，第二个指针就到了倒数第n个节点上。|
|LC109 Medium|使用快慢指针找链表中间节点，另外不一定需要nullptr表示链表结束，也可以再定义函数，让形参为左右两个指针表示左右边界来降低难度。|
|LC141 Easy| 环形链表，判断是否有环，使用快慢指针可以保证空间复杂度为O(1)，因为两者速度差一，使得当两个指针都进入环内的初始位置后，每一步都会使得两指针距离减一，推得如果有环那么肯定会相交。|
|LC142 Medium|环形链表，要找到环开始的节点，快慢指针很麻烦，使用vector来存节点，然后比较是否存在相同节点很简单，两者时间复杂度都是O(n)，不过这样空间复杂度是O(n)；如果要使用快慢指针，推出a=c+(n−1)(b+c)后，让新的指针从头走，slow指针接着走，发现当新指针走了a步，slow指针走了c步和n-1圈，刚好相交，这就是起始点，所以我们直接找新指针和slow指针第一次相交即可，此时就是起始点（https://leetcode-cn.com/problems/linked-list-cycle-ii/solution/huan-xing-lian-biao-ii-by-leetcode-solution/）。|

### **排序**

#### 简单的排序

1. 冒泡排序，复杂度O(n^2)，是一种稳定的排序（不会改变相同元素的相对位置）。
   
```C++

void bubble_sort(int arr[], int len) {
	int i, j, change=1;      
	for (i = 0; i < len - 1 && change != 0; ++i)
    {
        change=0;
		for (j = 0; j < len - 1 - i; ++j)
			if (arr[j] > arr[j + 1])
            {
				swap(arr[j], arr[j + 1]);
                change = 1;
        	}
    }
}

// 第一处是注意第二重循环只需要比较仍然无需的序列，也就是从 0 到 len - 1 - i
// 第二处是注意通过标志位，避免对排好序了的数组再无用的遍历下去

```

2. 插入排序

```c++

void insertion_sort(int arr[],int len){
        for(int i=1;i<len;i++){
                int key=arr[i];
                int j=i-1;
                while((j>=0) && (key<arr[j])){
                        arr[j+1]=arr[j];
                        j--;
                }
                arr[j+1]=key;
        }
}

// 原理就是将未排序序列的第一个元素（arr[i]），插入到已排序序列中合适的位置（通过key和从后往前的arr[j]的比较）

```

3. 选择排序 

#### 快速选择

<!-- 一般用于求解 Kth Element 问题，可以在 O(N) 时间复杂度，O(1) 空间复杂度完成求解工作。

与快速排序一样，快速选择一般需要先打乱数组，否则最坏情况下时间复杂度为 O(N2) -->

#### 堆排序

|题号|笔记|
|:-|:-|
|LC215 Medium|注意这种求第k大或者前k大的一般都可以用堆排序，利用小顶堆(升序)，当有限队列内元素数量超过k之后，删除掉堆顶这个最小的，最后剩下的就是那k个频率最大的元素了。|1
|LC451 Medium|注意这种有频率排序的可以用堆排序，先是哈希表统计频率，然后就是利用优先队列排序pair。|

#### 桶排序

|题号|笔记|
|:-|:-|
to do桶排序|LC347 Medium|求数组中频率前k高的元素集合，方法一：桶排序，；方法二：堆排序，利用小顶堆(升序)，当有限队列内元素数量超过k之后，删除掉堆顶这个最小的，最后剩下的就是那k个频率最大的元素了；方法三：哈希表按value排序，操作是map的所有pair元素放到一个集合，然后手写cmp比较pair类型，然后sort快排即可。|
|||

#### 归并排序

### **动态规划**

<!-- 递归和动态规划都是将原问题拆成多个子问题然后求解，他们之间最本质的区别是，动态规划保存了子问题的解。 -->

#### 简单一维DP问题

|题号|笔记|
|:-|:-|
|LC70 Easy|最简单的DP题，注意因为之前的数值不再被需要,所以可以用两个变量代替整个dp数组，只记录最新的两个楼梯爬法的数值，这样优化空间复杂度为O(1)，这就是滚动数组优化空间复杂度的思想，再注意一般一维dp数组都能通过这个滚动数组思想来优化空间。|
|LC198 Medium|找出递推关系有点难，另外注意dp\[i\]定义成偷前面i个屋子后，此时最多偷的金额，这个i代表前面i个屋，与第i个屋子本身偷不偷无关，是前面i个范围的最优解。|

#### 简单二维DP问题

|题号|笔记|
|:-|:-|
|LC62 Medium|判断出dp\[i\]\[j\] == dp\[i - 1\]\[j\] + dp\[i\]\[j - 1\]公式之后，发现初始值dp\[0\]\[j\] = 1 和 dp\[i\]\[0\] = 1，然后循环求值即可。|
|LC64 Medium|二维dp数组初始值最好设置\[i\]\[0\]和\[0\]\[j\],不要只给个\[0\]\[0\],这样两层for循环从i=1和j=1开始，完全避开了判断是否数组越界的情况，减少的代码量，另外本题关系dp\[i\]\[j\] = min(dp\[i - 1\]\[j\],dp\[i\]\[j - 1\]) + grid\[i\]\[j\]。|
|LC72 Hard|字符串编辑距离，看题解过的题。|


### **贪心思想**

水题：

|题号|笔记|
|:-|:-|
|LC455 Easy|要明白为了尽可能给更多人饼干，要给每个人尽量小的，另外要注意排序胃口，这样小胃口满足不了后面大胃口更满足不了，直接终止循环。|
|LC605 Easy||
  
### **二分思想**

水题：LC441 Easy

|题号|笔记|
|:-|:-|
|LC69 Easy|平方根不要再用循环一个一个试了，这个复杂度是O(根号X)；而使用二分则是O(logX)，快非常多。|
|LC378 Medium||
|||
|||

### **分而治之**

水题：

|题号|笔记|
|:-|:-|
|||
|||
|||
|||

### **数学问题**

#### 进制转换

|题号|笔记|
|:-|:-|
|LC405 Easy|int变量里面本身就是补码，直接位运算即可，从高到低每四位转换一个十六进制数，另外注意从高位到低位的顺序配合上字符串是否为空可以方便去除前导0，再注意下(char) ('a' + val - 10)映射。|
|LC504 Easy|采用模拟的思路，递归取余，不过首先注意负数的情况，再是注意0的情况，最后注意递归取的余数是从高位到低位的顺序。|

#### 阶乘

|题号|笔记|
|:-|:-|
|LC172 Medium|计算尾随零的数量，10的两个最基本的因子是2和5，我们只要分别计算能被2和5整除的次数，然后取min(num2,num5)即可得到可以凑出的10的个数，也就是尾随零的个数。|

#### 相遇问题

|题号|笔记|
|:-|:-|
|LC462 Medium||

### **LRU**

### **滑动窗口**
