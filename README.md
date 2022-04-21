# interview-note

目前正在找 C++ Linux 后端开发的实习，前途未卜中。

[简历 Resume](./src/resume_v2.pdf) (网页无法正常显示，需下载)

## 面试记录

### 23暑期实习(2022/3 - 2022/5)

简历挂：OPPO、携程、贝壳、网易、英雄游戏、拼多多

投递中：深信服、完美

|公司|岗位|笔试|一面|二面|三面|HR面|状态|
|:-|:-|:-|:-|:-|:-|:-|:-|
|米哈游|云游戏后台研发(C++)|4/10|X|X|X|X|笔试挂|
|百度|软件开发(C++)|4/12|4/17|X|X|X|面试挂|
|阿里|测试开发(C++)|4/15|-|-|-|-|笔试挂|
|DolphinDB|软件开发(C++)|作业|4/26|-|-|-|面试中|
|微众银行|数据库开发(C++)|4/20|-|-|-|-|笔试中|
|趋势科技|软件开发(C++)|4/22|-|-|-|-|笔试中|
|腾讯|软件开发(C++)|4/24|-|-|-|-|笔试中|
|字节游戏|软件开发(C++)|待定|-|-|-|-|笔试中|

### 23秋招提前批(2022/6 - 2022/7)

### 23秋招(2022/8 - 2022/9)

### 23春招(2023/2 - 2023/4)

## 实习经历

None

## 数据结构算法

1. 数据结构
    - [数组](#数组)
    - [栈](#栈)
      - [栈/递归](#栈递归)
      - [单调栈](#单调栈)
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
      - [拓扑排序](#拓扑排序)
    - [位运算](#位运算)

2. 算法
    - [搜索](#搜索)
      - [BFS/DFS](#BFSDFS)
      - [回溯](#回溯)
    - [排序](#排序)
      - [简单的排序](#简单的排序)
      - [快速选择](#快速选择)
      - [堆排序](#堆排序)      
      - [桶排序](#桶排序)
      - [归并排序](#归并排序)
    	<!-- 排序 https://leetcode-cn.com/problems/sort-an-array/solution/pai-xu-shu-zu-by-leetcode-solution/
    	快速选择
    	 -->
    - [双指针](#双指针)
    - [并查集](#并查集)
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
    - [滑动窗口](#滑动窗口)

3. 其他
	- [输入输出格式](#输入输出格式)

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

要求是每次入栈的元素必须要有序（如果新元素入栈不符合要求，循环将之前的元素出栈，直到符合要求再入栈），使之形成单调递增/单调递减的一个栈。当要入栈新元素的时候，我们发现如果栈不为空，那么这个栈顶元素就是我们要找的向前/向后（通过循环从前往后或者从后往前来控制）第一个大于/小于/大于等于/小于等于（通过改变内部入栈比较条件来控制）的元素，如果栈为空那么说明没有。关键词就是最近/后面/前面遇到的第一个的大于/小于/大于等于/小于等于的元素。

```c++

// 模板举例
while (!st.empty() && num[i] >= st.top())
    st.pop();

ans[i] = st.empty() ? -1 : st.top(); // code here 此处是通过栈顶元素获得结果的部分
st.push(num[i]);

```

<!-- to do Hard 84 85  -->

|题号|笔记|
|:-|:-|
|LC42 Hard|运用单调栈，每一次比较大于，得知有一个洼地，可能形成积水，通过不断出栈中间洼地或者比两端低的高墙的元素，计算新增加的积水(下层的积水之前已经计算完成)，然后把积水加起来即可。另外一点注意，因为我们要用索引做差求距离，所以单调栈里面存的是索引。|
|LC496 Easy|子问题是下一个更大的元素，明显是单调栈。然后总体问题通过哈希+单调栈解决，因为元素各异，所以可以用哈希表存结果。|
|LC503 Medium|循环数组拉直，破环成链，通过下标取模，使得范围从 0 到 2 * sz - 1 完成拉直工作，然后单调栈做法即可。第二点返回向量答案，但我们是逆序遍历得到答案的，为了避免逆序，我们定义vector<int> ret(sz)，直接初始化好对应的大小，逆序遍历直接通过索引ret\[i\] = x来存答案，避免多余的reverse函数。|
|LC739 Medium|单调栈问题，要注意一下这道题单调栈必须存索引，不能直接存元素，因为答案是索引做差得到的天数。|

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

水题：LC217 Easy

|题号|笔记|
|:-|:-|
|LC1 Easy|注意这道题是无序的，而且排序后下标也变了，所以这道题不是双指针类型的题，不要被误导。为实现O(log n)的复杂度，我们先使用哈希表记录下每个元素对应的下标，然后循环find得到target-nums\[i\]的迭代器，注意考虑重复元素的问题，判断条件中需要判断这两个不是同一个元素。|
|LC219|注意使用map记录元素和对应下标，另外注意如果第一个重复的元素不满足条件，那么应该更换映射的下标，因为随着向右遍历，离旧下标距离越来越远，根本不可能，所以一旦不满足，则换映射的下标。|

### **树**

#### 普通题型

水题：LC112 Easy、LC404 Easy、LC106 Medium、LC107 Medium、LC04.02 Easy

|题号|笔记|
|:-|:-|
|LC100 Easy|判断两个树是否相同，递归dfs，然后分类讨论p、q一个为空为假，两个都空为真，两个都不空就判断值即可。|
|LC101 Easy|对称，让根节点的左右节点，如此check(p->left, q->right) && check(p->right, q->left)递归比较；或者是先反转任一一个子树，然后递归比较相同。|
|LC103 Easy|bfs内部for循环来分层，加一个标志位控制插入deque的方向，用deque来存每层的结果，这样就不用reverse了。|
|LC104 Easy|求树的高度。|
|LC105 Medium|中序配合后序遍历生成二叉树，后序的最后一个节点是根节点(对应前序的第一个节点是根节点)，然后根据根节点值再中序遍历的位置，分割得到左右子树，然后递归处理。注意要点就是可以建立map，记录中序遍历值对应所在的下标，这样我们不用每次取得根节点值再搜索得到下标，而是可以直接从这个map得到。|
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
|LC199 Medium|二叉树的右侧视角(从右面看)，即每层最中最后的那个节点，层序遍历bfs即可完成，再记一下bfs内部for循环完成一层的遍历。同理左侧视角就是层序遍历每层最左面的节点了。|
|LC637 Easy|层次遍历，外层while条件是队列不为空，内部一个for循环是当前层的结点遍历，for循环的次数是通过队列size函数来得到的，即当前层结点数。|

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

#### 拓扑排序

|题号|笔记|
|:-|:-|
|LC207 Medium||
|LC210 Medium||
<!-- to do邻接矩阵的bfs和dfs实现，邻接链表的bfs和dfs实现 -->

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
|LC130 Medium|涉及到边界问题，这道题的问题在于要区分含不含边界两种情况，聪明的办法就是我们先只对边界dfs，将元素改成一种新的标记，即可区分包不包含边界的情况，最后再改回即可。|
|LC200 Medium|典型的棋盘问题，岛屿计数，遍历进行dfs，每一次遇到未访问的点加一计数即可。四个方向可以定义两个方向数组，写成循环减少代码量，而不用写四遍。|
|LC257 Easy|典型的树的递归遍历，注意通过参数来保存当前深度的已经过的路径，递归通过参数保存当前状态，和向下再传递状态。|
|LC386 Medium|按字典序用dfs遍历即可。|
|LC417 Medium|类似130题型，dfs倒序从边界遍历，即“水往高处流”，分别遍历两种海洋，最后取交叉即可。|
|LC547 Medium|题目很简单，是典型的邻接矩阵格式的图的dfs遍历写法；另外用bfs当然也可以，通过循环+队列实现。|
|LC695 Medium|典型的棋盘问题，另外通过递归返回值作为结果可以减少代码量，尽量用这个。|
|LC733 Medium|如果用bfs写，注意bfs和dfs一样需要vis标志是否访问过，不过很多题中不需要显式vis标记，比如这道题可以通过设置image\[x\]\[y\]为newColor隐式标记这个点已经访问过，如果不能标记，那么还是接着用vis标记即可。另外这道题注意判断新旧颜色相同的情况，避免造成无限递归。|

#### 回溯

回溯算法是对树形或者图形结构执行一次深度优先遍历，在遍历的过程中寻找问题的解。当发现已不满足求解条件时，就返回，尝试别的路径。此时对象类型变量就需要重置成为和之前一样，称为“状态重置”，而这就是与DFS的区别。许多复杂的，规模较大的问题都可以使用回溯法，实际上，回溯算法就是暴力搜索算法。

DFS使用的标记数组是不需要状态重置的，因为这是一个全局状态，全局只走一遍，比如遍历，从一个结点向右先走的路径，之后再从这个点向下走就不可能再走了，因为两个方向都是服务于同一个目标；而回溯不一样，对于回溯来说向右走的尝试和之后向下走的尝试是没有关系的，之后向下走完全是一个新的尝试，不受之前向右走尝试的影响，所以需要向右走完之后撤销状态。

状态撤销有两种，一种是自然的撤销操作，另一种是通过传值，使得递归调用的子函数改变的是副本从而不会影响到另一种尝试，当然传值拷贝的费用不低，所以要优先使用撤销的方式，这样我们传递引用避免拷贝的成本，而且可读性更好，用的人更多。

如果看见排序，组合，枚举，暴力搜索或类似的列出全部可能性的题，一般就是回溯题型了。

水题：to do 撤销操作完成状态重置LC17 Medium

|题号|笔记|
|:-|:-|
|LC39 Medium|、|
|LC40 Medium|、|
|LC46 Medium|不重复数组的全排列，我们要在每层取出一个数作为第一个数放入一个临时向量，然后递归子函数在剩余部分再选出下层的第一个数，递归即可，最后把这个临时向量放入答案，我们只需要额外定义一个 vis 向量来监视是否上层函数已经选过这个数当作第一个数即可实现，记得本层函数结束后要撤销对 vis 的操作；首先要注意得到剩余部分的时候，不要从向量中删除，因为会导致删除元素右面的所有元素的移动，代价太大；然后还有一个优化是通过交换来实现取本层的数，可以不用定义 vis 向量，然后 start + 1 来告知递归子函数要处理的剩余部分，注意本层交换操作回溯回来要撤销操作来重置状态，来将我们传递引用的nums数组恢复到操作前的状态。|
|LC47 Medium|可重复数组的全排列，剪枝优化。涉及考虑重复元素，或者大小比较的情况，答案顺序任意，一定要考虑下排序，排序之后就可以更好的找到规律，更好找到剪枝等优化操作的规律。答案顺序任意，因而这道题排序后答案没问题，很容易发现，在同层中，重复元素选任何一个的结果都一样，从而找到剪枝条件(i > 0 && !vis\[i - 1\] && nums\[i\] == nums\[i - 1\])。|
|LC51 Hard|N皇后问题，一行必须放一个，所以可以把行数当下标，然后列、左斜、右斜三个集合查集合回溯即可。注意通过行数下标减去列，可以作为左斜的集合的元素。|
|LC77 Medium|组合问题，从n个数里面选k个，列出全部组合情况（一种组合中内部的顺序无意义），|
|LC78 Medium|、|
|LC79 Medium|、|
|LC90 Medium|、|
|LC93 Medium|字符串dfs遍历所有可能性即可，注意前导0格式问题。|
|LC113 Medium|树的路径问题，注意我们无需对每一次路径都值传递拷贝一次，我们只需要一个全局的 vector<int> path 记录路径，进入就 push，离开就 pop，只需要这一个就可以完成记录路径的作用了。|
|LC131 Medium|、|
|LC216 Medium|、|
|LC526 Medium|、|

### **双指针**

167 345 633 680 88 524 \ 11 15 31 42 75 148 253 581

|题号|笔记|
|:-|:-|
|LC19 Medium|链表倒数第n个节点，先让一个指针指向正向第n个，然后另一个指针指向头结点，当第一个指针到末尾时，第二个指针就到了倒数第n个节点上。|
|LC109 Medium|使用快慢指针找链表中间节点，另外不一定需要nullptr表示链表结束，也可以再定义函数，让形参为左右两个指针表示左右边界来降低难度。|
|LC141 Easy| 环形链表，判断是否有环，使用快慢指针可以保证空间复杂度为O(1)，因为两者速度差一，使得当两个指针都进入环内的初始位置后，每一步都会使得两指针距离减一，推得如果有环那么肯定会相交。|
|LC142 Medium|环形链表，要找到环开始的节点，快慢指针很麻烦，使用vector来存节点，然后比较是否存在相同节点很简单，两者时间复杂度都是O(n)，不过这样空间复杂度是O(n)；如果要使用快慢指针，推出a=c+(n−1)(b+c)后，让新的指针从头走，slow指针接着走，发现当新指针走了a步，slow指针走了c步和n-1圈，刚好相交，这就是起始点，所以我们直接找新指针和slow指针第一次相交即可，此时就是起始点（https://leetcode-cn.com/problems/linked-list-cycle-ii/solution/huan-xing-lian-biao-ii-by-leetcode-solution/）。|

### **并查集**

|题号|笔记|
|:-|:-|
|LC399 Medium||

### **排序**

#### 简单的其他排序

1. **冒泡排序**，复杂度O(n^2)，是一种稳定的排序（不会改变相同元素的相对位置）。每一轮内部循环通过交换把一个最大的元素交换到最后，形成{未排序|排序}，然后重复这个过程。
   
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

2. **选择排序**，复杂度O(n^2)，不稳定。{排序|未排序}，每次内部循环未排序部分选择到一个最小的元素，然后通过交换把这个最小的元素与未排序的第一个元素交换，重复这个过程。

```c++

void selection_sort(vector<int>& arr) {
        for (int i = 0; i < arr.size() - 1; i++) {
                int min = i;
                for (int j = i + 1; j < arr.size(); j++)
                        if (arr[j] < arr[min])
                                min = j;
                swap(arr[i], arr[min]);
        }
}

```

3. **插入排序**，复杂度O(n^2)，稳定。{排序|未排序}，每次内部循环，把未排序的第一个元素暂存，然后不懂右移排序部分的元素，直到找到合适的位置，把这个未排序的第一个元素插入到这里，重复这个过程。

```c++

void insertion_sort(int arr[],int len)
{
        for(int i = 1; i < len; ++i)
		{
                int key = arr[i];
                int j = i - 1;
                while((j >= 0) && (key < arr[j]))
				{
                        arr[j + 1] = arr[j];
                        --j;
                }
                arr[j + 1] = key;
        }
}

// 原理就是将未排序序列的第一个元素（arr[i]），插入到已排序序列中合适的位置（通过key和从后往前的arr[j]的比较）

```

4. **希尔排序**，复杂度O(n*log(n)^2)或n^(3/2)，不稳定。

5. 其他排序：
   - 计数排序，元素是n个0到k之间的整数时，它的时间复杂度是Θ(n + k)，空间复杂度是O(k)，显然当n很小k很大的时候，比如比较1和10000000，这个算法既浪费了几乎全部开辟的空间，又慢的令人发指（因为需要遍历开辟的数组来排序，比较无数个arr\[i\]是否为0的情况），在绝大多数情况下是个无用的算法。仅仅在k很小，且n较大，分布比较均匀才有用。
   - 基数排序整数，按位数切割成不同的数字，然后按每个位数分别放入桶中进行排序，循环每一位进行比较。

#### 快速排序

|题号|笔记|
|:-|:-|
|LC912 Medium|时间复杂度O(log n)，这是因为内部递归不断分别处理左右两个子序列。貌似面试考的次数挺多的，一定要会写。|

```c++
class Solution
{
public:
	vector<int> sortArray(vector<int> &nums)
	{
		quickSort(nums, 0, nums.size() - 1);
		return nums;
	}

private:
	void quickSort(vector<int> &nums, int l, int r)
	{
		if (l < r)
		{
			int index = partition(nums, l, r);
			quickSort(nums, l, index - 1);
			quickSort(nums, index + 1, r);
		}
	}

	int partition(vector<int> &nums, int l, int r)
	{
		int p = (rand() % (r - l + 1)) + l;
		swap(nums[l], nums[p]);
		int s = nums[l];
		int start = l;
		while (l < r)
		{
			/* 这里一定要先写 --r，因为开头的基准可以作为哨兵，当之后所有元素都
			 * 小于基准的时候，r 会变成 start 正常停止；
			 * 而如果先写 ++l，当基准后所有元素都小于等于基准时，l 的值就会变成
			 * 最后一个小于等于的元素下标，和其他情况下的 l 终止状态(第一个大于
			 * 的下标)不一样，导致错误。
			 */
			while (l < r && nums[r] > s)
				--r;
			while (l < r && nums[l] <= s)
				++l;
			if (l < r)
				swap(nums[l], nums[r]);
		}
		swap(nums[start], nums[r]);
		return l;
	}

	void swap(int &left, int &right)
	{
		int t = left;
		left = right;
		right = t;
	}
};
```

#### 堆排序

水题：LC703 Easy

|题号|笔记|
|:-|:-|
|LC215 Medium|注意这种求第k大或者前k大的一般都可以用堆排序，利用小顶堆(升序)，当有限队列内元素数量超过k之后，删除掉堆顶这个最小的，最后剩下的就是那k个频率最大的元素了。|
|LC451 Medium|注意这种有频率排序的可以用堆排序，先是哈希表统计频率，然后就是利用优先队列排序pair。|

#### 桶排序

|题号|笔记|
|:-|:-|
to do桶排序|LC347 Medium|求数组中频率前k高的元素集合，方法一：桶排序，；方法二：堆排序，利用小顶堆(升序)，当有限队列内元素数量超过k之后，删除掉堆顶这个最小的，最后剩下的就是那k个频率最大的元素了；方法三：哈希表按value排序，操作是map的所有pair元素放到一个集合，然后手写cmp比较pair类型，然后sort快排即可。|
|||

#### 归并排序

|题号|笔记|
|:-|:-|
|LC23 Hard||
|LC148 Medium||
|LC912 Medium||

### **动态规划**

<!-- 递归和动态规划都是将原问题拆成多个子问题然后求解，他们之间最本质的区别是，动态规划保存了子问题的解。 -->

|LC279 Medium|todo凑数问题，同样是用其他数值凑出目标值，问最少需要多少个数，和LC322思路相同。|
|LC322 Medium|todo零钱兑换，也是凑数问题，问用其他价格的硬币为了凑出目标值，最小需要多少枚。这道题看起来用暴力搜索做很容易，但是这样会超时，所以还是要用DP的思路。|

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

注意二分查找的前提是向量已排好序，所以如果题目告诉你已排好序(或者你需要手动排序)，然后查找某一个元素或者重复元素的最左、最右侧的元素，可以使用二分查找。

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

|题号|笔记|
|:-|:-|
|LC146 Medium|LRU(Least Recently Used)最近最少使用，关键是按上一次的访问时间排序，然后逐出的是最久未访问的数据。使用的是一个叫哈希链表的数据结构，即哈希表和双向链表，完成get和put都是O(1)的复杂度。注意unordered_map才是基于哈希的，map是基于红黑树的，查询复杂度是O(log n)。|
|LC460 Hard|LFU()|

### **滑动窗口**

<!-- 76 239 438 -->
|题号|笔记|
|:-|:-|
|LC3 Medium|、|
|LC438 Medium|这道题简单，因为窗口的长度是固定的，我们只需要不断右移整个窗口，并对比这26个字母个数是否相同即可。可以使用 unordered_map 来比较，但一定要注意，map\[index\] 值为0和不存在这个 key 的元素是不一样的；也可以用vector来比较，下标对应字母即可。|

### **输入输出格式**

1. 目前用到的笔试是牛客网的在线笔试，https://www.nowcoder.com/test/27976983/summary#question 这个网站可以进行输入输出格式练习。
2. 获取一行用getline(输入流，字符串引用)（默认分割符为'\n'），然后通过stringstream字符流来输出分隔空格，如果要分隔的是别的符号比如','，那么就要再次getline(stringstream流，字符串引用，分割符)来分隔，因为c++标准库里面没有split，所以要通过这种方式来实现。
3. 注意审题，是多组数据还是一组数据，有时候题干是多组数据但示例给的只有一组，让人误解。
4. 注意审题数据范围，INT32_MAX 是 2^31-1 在 10^9 ~ 10^10 范围，别忘了改用 long，如果特意说是32位机器，long 和 int同大小要用 long long。
5. 另外是输出末尾不让有空格之类的问题，一般我们循环外输出第一个元素（别忘了额外判断是否为空），之后循环中输出空格加剩下元素即可。
6. 注意使用英文字符。
7. cin.ignore(n,终止字符) 函数作用是跳过输入流中n个字符，或在遇到指定的终止字符时提前结束(含终止字符)。cin 和 getline 混用，要注意 cin 按类型输入完会留下'\n'，所以我们调用一个 cin.ignore() 即可，默认参数是 n=1 忽略一个 '\n' 即可。