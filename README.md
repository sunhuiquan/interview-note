# interview-note

## 目前状况

等待智臾的暑期实习入职，紧张ing。开始刷 CMU 15-445 和 MIT 6.824，希望自己不是实习生里面最菜的。。。然后坚持刷 Leetcode 和打周赛，日常复习面经，准备秋招提前批投投大厂锻炼下。

[最新简历 Resume](./src/resume_v1.pdf) (网页可能无法正常显示，需下载)

---

## 面试记录

### 23秋招提前批 (2022/6 - 2022/7)

简历挂：-

投递中：-

|公司|岗位|笔试|一面|二面|三面|HR面|状态|
|:-|:-|:-|:-|:-|:-|:-|:-|

### 23暑期实习 (2022/3 - 2022/5)

简历挂：OPPO、携程、贝壳、网易、英雄游戏、拼多多、深信服

投递中：完美、七牛云、虹软、搜狐畅游、毫末智行、淘乐、哲库科技、CVTE、海信、快手、美团、旷视

|公司|岗位|笔试|一面|二面|三面|HR面|状态|
|:-|:-|:-|:-|:-|:-|:-|:-|
|米哈游|云游戏后台研发(C++)|4/10|X|X|X|X|笔试挂|
|阿里|测试开发(C++)|4/15|X|X|X|X|笔试挂|
|百度|软件开发(C++)|4/12|4/17|X|X|X|面试挂|
|趋势科技|软件开发(C++)|4/22|X|X|X|X|笔试挂|
|腾讯|软件开发(C++)|4/24|X|X|X|X|笔试挂|
|微众银行|数据库(C++)|4/20|-|-|-|-|等待|
|京东|测试开发(C++)|4/29|-|-|-|-|等待|
|海康威视|软件开发(C++)|4/30|-|-|-|-|等待|
|XSKY|软件开发(C++)|5/7|-|-|-|-|等待|
|美的|软件开发(C++)|5/14|-|-|-|-|笔试|
|智臾|软件开发(C++)|4/20|4/26|5/7|-|-|已接受OFFER|

---

## 工作经历

### 浙江智臾科技（2022/7 - 2023/1）

职位：C++后端开发实习生  
内容：-

---

## 数据结构算法

1. 数据结构
    - [数组](#数组)
    - [栈](#栈)
      - [栈/递归](#栈递归)
      - [单调栈](#单调栈)
    - [队列](#队列)
      - [普通队列](#普通队列)
      - [堆(优先队列)](#堆优先队列)
	  - [单调队列](#单调队列)
    - [链表](#链表)
    - [字符串](#字符串)
      - [常规字符串](#常规字符串)
      - [大数问题](#大数问题)
    - [哈希表](#哈希表)
    - [树](#树)
      - [普通题型](#普通题型)
      - [遍历](#遍历)
      - [BST(二叉搜索树)](#BST(二叉搜索树))
      - [Trie(字典树、前缀树)](#Trie(字典树前缀树))
      - [N叉树](#N叉树)
    - [图](#图)
      - [拓扑排序](#拓扑排序)
    - [位运算](#位运算)

2. 算法
    - [搜索](#搜索)
      - [DFS](#DFS)
      - [BFS](#BFS)
      - [多源BFS](#多源BFS)
      - [回溯](#回溯)
	- [双指针](#双指针)
    - [排序](#排序)
      - [简单的排序](#简单的其他排序)
      - [快速排序](#快速排序)
      - [堆排序](#堆排序)      
      - [桶排序](#桶排序)
      - [归并排序](#归并排序)
    	<!-- 排序 https://leetcode-cn.com/problems/sort-an-array/solution/pai-xu-shu-zu-by-leetcode-solution/
    	快速选择
    	 -->
    - [并查集](#并查集)
    - [动态规划](#动态规划)
      - [简单一维DP问题](#简单一维DP问题)
      - [简单二维DP问题](#简单二维DP问题)
      - [数组区间问题](#数组区间问题)
      - [字符串问题](#字符串问题)
      - [最长递增子序列](#最长递增子序列)
      - [最长公共子序列](#最长公共子序列)
      - [数值拆分问题](#数值拆分问题)
      - [背包问题](#背包问题)
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

水题：LC108 Easy、LC118 Easy、LC240 Medium、LC448 Easy、LC485 Easy、LC566 Easy、LC654 Medium、LC697 Easy、LC766 Easy、LC1984 Easy

|题号|笔记|
|:-|:-|
|LC283 Easy|双指针交换类型，注意当左索引没有指0的时候，左右索引相同，交换的是自身。|
|LC287 Medium||
|LC303 Easy|记录从下标 0 到每个元素的数值和，这样一个区间内的数值和就可以转换为两个数值和的差值。|
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

有时候我们需要比较自定义类型，这个时候需要我们手写比较类，下面是一个例子。

```c++
struct cmp
{
	bool operator()(const ListNode *l1, const ListNode *l2)
	{
		return l1->val > l2->val;
	}
};

//使用时
priority_queue<ListNode *, vector<ListNode *>, cmp> pq;
```

prob 218 264

跳转：[堆排序](#堆排序)

#### 单调队列

|题号|笔记|
|:-|:-|
|LC239 Hard|to do|
|LC264 Medium|直接从堆里取出一个，然后加入 2x、3x、5x，这个的正确性是没问题的，因为当前点的下一层的三个点一定比这个大，排除，所以可以得证。另外注意，为了去重，我们使用 unordered_set，不断插入，如果 count 发现已存在，则直接丢弃。|
|LC1438 Medium|to do|

### **链表**
   
水题：LC2 Medium、LC21 Easy、LC24 Medium、LC83 Easy、LC206 Easy、LC328 Medium、LC725 Medium

|题号|笔记|
|:-|:-|
|LC160 Easy|两个链表相交点，不使用额外空间的方法，就是利用 a+b+c = c+b+a，其中b是相交之后共同的部分，a和c分别是两个链表相交之前的部分，利用这个数学原理得到，走完a+b+c后，下一个一定是b的第一个，也就是相交的第一个节点；另外如果两个链表不相交，那么就是a+c=c+a，下一个都是nullptr，虽然不相交但也可以通过 l1 == l2 来终止循环,无需额外考虑。|
|LC234 Easy|不使用额外空间的做法，快慢指针找中间节点，然后后半个链表再反转（使用前中后三个节点来实现反转），然后比较这两个链表。
|LC445 Medium|这道题本身很简单，难点在于得到逆序的数值，关于逆序我们首先就要想到栈，使用栈很简单，比反转链表方便多了。|

### **字符串**

#### 常规字符串

水题：LC6 Medium、LC9 Easy、LC28 Easy、LC205 Easy、LC242 Easy、LC409 Easy

|题号|笔记|
|:-|:-|
|LC49 Medium|注意包含相同字母(个数也相同)的单词，可以经过排序，得到一个相同的单词作为key，然后使用哈希。|
|LC266 Easy|利用回文字符串的特性+哈希表，奇数次数的字符小于等于 1 则可以通过排列变成回文数。|
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
|LC219 Easy|注意使用 unordered_map 记录元素和对应下标，另外注意如果第一个重复的元素不满足条件，那么应该更换映射的下标，因为随着向右遍历，离旧下标距离越来越远，根本不可能，所以一旦不满足，则换映射的下标。|
|LC594 Easy|哈希表直接通过 key + 1 找到目标的元素。|

### **树**

#### 普通题型

水题：LC112 Easy、LC404 Easy、LC106 Medium、LC107 Medium、LC04.02 Easy、LC563 Easy

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

prob 297 331

水题：LC94 Easy、LC144 Easy、LC145 Easy、LC700 Easy、LC513 Medium、LC671 Easy

|题号|笔记|
|:-|:-|
|LC74 Medium|从右上角来看，你会发现这是一个BST，初始定义右上角坐标，然后循环比较 target 比较即可。|
|LC173 Medium|笨办法需要 O(n) 复杂度，中序遍历结果放到向量里即可，不需要解释。另一种方式是通过栈的思路，to do|
|LC199 Medium|二叉树的右侧视角(从右面看)，即每层最中最后的那个节点，层序遍历bfs即可完成，再记一下bfs内部for循环完成一层的遍历。同理左侧视角就是层序遍历每层最左面的节点了。|
|LC270 Easy|查找离目标值最近的节点值，注意目标值是 double，所以要小心类型问题。另外 abs() 返回的是 int 类型，所以不能用在 double 上，需要手动去绝对值。|
|LC606 Easy|这道题要利用递归函数的返回值，返回 string 代表处理的该个子树的结果，这样就可以把一个父问题变为处理 root 和两个子树的递归问题了，注意要用 string 返回值代表处理的子问题的结果，这样写起来会很简洁。|
|LC637 Easy|层次遍历，外层while条件是队列不为空，内部一个for循环是当前层的结点遍历，for循环的次数是通过队列size函数来得到的，即当前层结点数。|
|LC637 Easy|层次遍历，外层while条件是队列不为空，内部一个for循环是当前层的结点遍历，for循环的次数是通过队列size函数来得到的，即当前层结点数。|
|LC783 Easy|利用 BST 中序遍历的性质，直接在递归中做差取最小值即可。|
|LC872 Easy|遍历树，内部用 !r->left && !r->right 对每一次递归的子树根节点判断是否是叶节点即可。|
|LC897 Easy|笨办法是先中序遍历把每一个 root 存到 vector 里，然后再循环 vector 来构造。另一种方法不需要额外的空间复杂度，那就是中序遍历的时候记录上一个节点，让上一个节点的 right 是这一个节点即可，但这样我们需要额外考虑第一个节点的情况，一种好的方法是一开始 new 一个 dummy 节点作为前一个结点即可，这样不用考虑前一个结点为 nullptr 的情况。|
|LC993 Easy|找到父节点和深度然后对比即可，可以使用参数来传递父节点，如果通过给一个变量不断赋值来记录父节点，要注意当找到答案后立马返回，避免又修改了。|
|LC938 Easy|注意这是BST，递归思路，如果 root->val > high 则只可能左子树，反之则只能右子树，然后就是 root->val 和左右子树。|

#### BST(二叉搜索树)

|题号|笔记|
|:-|:-|
|LC230|BST按中序遍历会得到一个递增的序列，中序遍历+中间计数即可求得BST中第k小的元素了。因为中序遍历有个特性是先到最左面的叶子结点，那么复杂度是O(H+k)，H是树的高度，所以如果我们将BST优化成AVL能把复杂度最小化成O(log N + k)，因为AVL树的高度是log N。|
|LC501|利用BST中序遍历是非递减序列完成，注意通过非递减这个特性，我们可以不用哈希表，从而空间复杂度为O(1)。|
|LC530 Easy|注意这是BST，所以一定有什么寓意，发现是利用BST最重要的那个性质，中序遍历得到非递减序列，依次做差即可。另外要注意 prev 一开始不能直接用，要一个 bool 变量标记，当 prev 被赋值后，更改那个 bool 标记才能用。|
|LC538 Medium|这道题使用了逆中序遍历的思路，先右再左遍历。既然中序遍历是递增，那么逆中序自然是递减，方向是从右往左，累加和也变得可以计算了。|
|LC653 Easy|用烂了的BST中序遍历+双指针即可。|

#### Trie(字典树、前缀树)

|题号|笔记|
|:-|:-|
|LC139 Medium||
|LC208 Medium|典型的字典树写法，另外注意下利用是否是叶节点的布尔变量来区分查前缀和搜索单词。|
|LC677 Medium|字典序前缀和的问题，注意可以优化每个节点保存一个前缀和的方式，通过每次新插入增加路径前缀节点的值；另外注意通过哈希表记录以及插入的值，发生改变值的时候，我们就这次新值和之前值做差，然后增加前缀节点这个差值，完成更改。|

#### N叉树

水题：LC589 Easy、LC590 Easy

|题号|笔记|
|:-|:-|
|LC429 Medium|N叉树层序遍历，和常规的二叉树的层次遍历方法几乎一样，同样注意 while 内部用 for 一次遍历一层。|

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

#### DFS

prob 1723 1766

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
|LC797 Medium||
|LC1020 Medium|记住，棋盘的问题，如果题干和边缘相关，那么我们就要考虑只对边界的点遍历即可的思路。我用的是笨办法，对每个点 dfs 然后看标志位是否经过边界，如果没有经过则 ret 加上 dfs 返回的经过的块数，这种方法写起来挺恶心的。而一个聪明的方法是只对边界的陆地点 dfs，然后把这些和边缘连通的陆地变成 2，最后记录 1 的数量即可。|
|LC1034 Medium|注意如果直接在 dfs 中更改颜色，则会影响后序的判断，所以我们需要先记下所有要改的位置，但先不改，到最周再一起改。|
|LC2049 Medium|to do|

#### BFS

<!-- 建图法 -->

prob 127 403 752 773 778 815 847 909 1034 1036 1345 1631 2039 2045 2059 LCP07

|题号|笔记|
|:-|:-|
|LC863 Medium|求距离为 k 的点我们很容易想到 BFS，我们但是这个题是树而不是图，所以我们只需要通过树构造一个图即可，图我们使用邻接表的形式 vector<vector<int>>，首先来个先序遍历，对每个结点判断是否有子节点，有的话把这个边加入无向图(两个方向都加入)即可，不用担心加重复的问题，因为每个结点只能加它和它的子节点的边，不会加往上的边，所以不可能重复加入。构建好图后然后就对图用 BFS 遍历，依旧是内部 for 循环一层(注意同一层距离相同)的全部元素即可。|

#### 多源BFS

|题号|笔记|
|:-|:-|
|LC1162 Medium|to do|
|LC1765 Medium|典型的多源 BFS 题型，注意我们对 0 BFS 遍历得到所有 1 的位置，剩下的与 1 相邻位置就必须是 2 了，因为所有必须是 1 的位置已经标出，这保证了正确性，BFS 遍历，直到队列为空即可。|

#### 回溯

<!-- 37 52 139 140 212 301 1255 -->
<!-- 31 264 -->

回溯算法是对树形或者图形结构执行一次深度优先遍历，在遍历的过程中寻找问题的解。当发现已不满足求解条件时，就返回，尝试别的路径。此时对象类型变量就需要重置成为和之前一样，称为“状态重置”，而这就是与DFS的区别。许多复杂的，规模较大的问题都可以使用回溯法，实际上，回溯算法就是暴力搜索算法。

DFS使用的标记数组是不需要状态重置的，因为这是一个全局状态，全局只走一遍，比如遍历，从一个结点向右先走的路径，之后再从这个点向下走就不可能再走了，因为两个方向都是服务于同一个目标；而回溯不一样，对于回溯来说向右走的尝试和之后向下走的尝试是没有关系的，之后向下走完全是一个新的尝试，不受之前向右走尝试的影响，所以需要向右走完之后撤销状态。

状态撤销有两种，一种是自然的撤销操作，另一种是通过传值，使得递归调用的子函数改变的是副本从而不会影响到另一种尝试，当然传值拷贝的费用不低，所以要优先使用撤销的方式，这样我们传递引用避免拷贝的成本，而且可读性更好，用的人更多。

如果看见排序，组合，枚举，暴力搜索或类似的列出全部可能性的题，一般就是回溯题型了。

另外如果 TLE 考虑一下能不能剪枝(避免根本不可能得到答案的递归调用)，若是还不行就转为记忆化搜索，然后还可以转 DP 来做。

水题：LC17 Medium

|题号|笔记|
|:-|:-|
|LC31 Medium|to do返回当前排列的下一个字典序排列。|
|LC37 Hard|to do|
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
|LC267 Medium|、|
|LC306 Medium|暴力回溯思路，但是由于数值可能很大，long long 也无法满足，所以我们可以通过 vector<int> 从低位到高位(这样进位好写，如果最后有进位的多一位的 1 可以直接加在最后，不用全体右移一位)存入数值的每一位，然后实现一个函数实现两个这个相加和比较，本质和 string 加法一样，就是方便一些，除了有这个溢出问题，这道题和 LC842(这道题告诉你最多 32 位来保证了用 long long 完全足够，超过 INT32_MAX 直接舍弃，不会累加超出 long long 的范围) 本质一样。|
|LC401 Easy|这道题是简单题是因为数值范围很小，我们可以直接穷举，算出对应的小时和分钟位数相加如果等于则 ++ret 即可。常规思路是回溯，中等难度，难点在于我们不能把小时位数和分钟分数分开想，要合起来想，想成一共 10 位，前 4 位代表小时，然后回溯把这一共的 10 位进行 k 次置位成 1，然后对每个结果的位数分析得到对应字符串。|
|LC526 Medium|、|
|LC797 Medium|这道题因为告诉你是有向无环图，所以非常简单，直接回溯做出来即可。|
|LC842 Medium|回溯参数是新一个数字的开始下标，然后循环取该层的数值 n = n * 10 + num\[start + i\]，注意如果是 0 那么只有一种可能，如果不满足要求要提前退出。|
|LC1219 Medium|多源 DFS，对每一个非 0 的点进行回溯暴力做即可，另外这道题这么高的复杂度竟然没有超时。|
|剑指38 Medium|字符串的排列，注意 string 和 vector<T> 很像，都有 push_back() 和 pop_back()，所以完全可以看成 vector<char> 即可。典型的题，vis 表示字符串的某位置元素是否已选取(不可重复)，然后对于重复的答案，我们使用 set 来存，最终再用 set 构造 vector 返回即可。可以剪枝提高效率，先要排序(答案顺序任意所以不影响)，这样让重复的元素在一起，这是最关键的一步，然后同层的重复元素我们选取最左面一个即可，最左面的那个的它自己的 vis 直接是 1 然后向下递归回溯，而后面的重复元素分成两种情况，如果是前一种情况的子递归，那么 vis\[i - 1\] 一定是 1，而如果是同层的重复元素，会发现 vis\[i - 1\] 是 0，因为前一个元素已经回溯完成了，1 又变回了 0，此时无需再次选中这个重复的元素了，剪枝即可。另外这道题更好的思路是用 LC31 的下一个字典序排列。|

### **双指针**

167 345 633 680 88 524 \ 11 15 31 42 75 148 253 581

|题号|笔记|
|:-|:-|
|LC19 Medium|链表倒数第n个节点，先让一个指针指向正向第n个，然后另一个指针指向头结点，当第一个指针到末尾时，第二个指针就到了倒数第n个节点上。|
|LC56 Medium|首先排序，使得可能合并的元素靠在一起，这样才能遍历合并区间。然后根据后一个的 \[0\] 和 前一个的 \[1\] 比较合并即可，合并成的放入结果即可。可以使用双指针，一个是正在合并的元素，这个元素随着合并更改边界，另一个是待判断合并的元素，如果不能合并，则 a = b 开始下一轮的合并区间和 push_back 上个合并得到的结果即可。|
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

```c++
void MergeSort (int arr [], int low,int high) {
    if(low>=high)
		return;
    int mid =  low + (high - low) / 2;
    MergeSort(arr,low,mid);
    MergeSort(arr,mid+1,high);
    merge(arr,low,mid,high);
	// 合并两个有序序列，中间需要额外空间存排序后的结果，然后再拷贝回原数组
}
```

|题号|笔记|
|:-|:-|
|LC23 Hard|把 k 个链表一开始 push 进优先队列，之后循环取最小并如有下一个把下一个也 push 进入。注意对于自定义类型我们要自己手写比较类，然后注意为了避免讨论头指针，我们创建了一个 dummy 无用结点，最后返回 dummy.next 即可。|
|LC148 Medium||
|LC912 Medium||
|LC1305 Medium|弱智的中序+归并做出，为什么用归并呢，因为中序后我们得到了两个分别有序的向量，显然归并 O(m + n)，像快排就无法用到已经有序这个属性，复杂度 O(nlogn)。|

### **动态规划**

prob 3 10 22 32 53 121 139 338 647

递推公式可能难以想出，我们可以先写回溯递归算法，然后改成记忆化搜索(注意递推改成记忆化搜索需要递推函数通过返回值返回结果才可)，最后在记忆化搜索的代码中推出 DP 即可(也可以直接记忆化搜索完事)。

#### 简单一维DP问题

水题：LC509 Easy

|题号|笔记|
|:-|:-|
|LC53 Easy|注意这个题干要求是连续的，意思是 f(i) 必须代表着以 nums\[i\] 结尾的最好结果，这样后一个和前一个元素才能连在一起符合题意连续的序列，不能间断。f(i) = max(f(i - 1) + nums\[i\], nums\[i\])，一个是继续连续下去以 nums\[i\] 结尾的最好结果，一个是作为新的连续序列的开头。|
|LC70 Easy|f(i) = f(i-2) + f(i-1) 最简单的DP题，注意因为之前的数值不再被需要,所以可以用两个变量代替整个dp数组，只记录最新的两个楼梯爬法的数值，这样优化空间复杂度为O(1)，这就是滚动数组优化空间复杂度的思想，再注意一般一维dp数组都能通过这个滚动数组思想来优化空间。|
|LC120 Medium|、|
|LC198 Medium|f(i) = max(f(i-2) + nums\[i\], f(i - 1)) 找出递推关系有点难，另外注意dp\[i\]定义成偷前面i个屋子后，此时最多偷的金额，这个i代表前面i个屋，与第i个屋子本身偷不偷无关，是前面i个范围的最优解。|

#### 简单二维DP问题

|题号|笔记|
|:-|:-|
|LC62 Medium|f(i,j) = f(i-1,j) + f(i,j-1)，发现初始值 f(0,j) = 1 和 f(i,0) = 1，分补对应一直向右和一直向下，只有一种方式，然后循环求值即可。|
|LC64 Medium|f(i,j) = min(f(i-1,j), f(i,j-1)) + grip\[i\]\[j\]，二维dp数组初始值最好设置\[i\]\[0\]和\[0\]\[j\],不要只给个\[0\]\[0\],这样两层for循环从i=1和j=1开始，完全避开了判断是否数组越界的情况，减少的代码量.。|

#### 数组区间问题

|题号|笔记|
|:-|:-|
|LC413 Medium||

#### 字符串问题

|题号|笔记|
|:-|:-|
|LC5 Medium|回文串的性质很容易发现 f(i,i) = true 和 f(i,j) = f(i+1,j-1)，比较难想的是我们要用二维 DP，i 表示子串开头，j 表示字串结尾。另外要注意写循环的顺序，因为是 i+1 和 j-1，所以是从左下方位得到，所以我们循环要先得到左边的列，再得到右面的列，所以我们内部循环是 i，外侧可以循环子串的长度。|
|LC72 Hard|这道题很难直接想出来状态转移方程，那么我们不如直接暴力回溯做出来一个超时的版本，然后选取合适的(这道题是两个字符串各自当前的长度)状态写一个数组，记录下答案，如果在表中已经计算过就直接取出答案，以此避免重叠子问题的计算。最后根据递归的规律可以再自底向上通过动态规划的方式(DP表)写出来，或者干脆直接用刚才记忆化搜索的方式提交均可。总而言之，要是能直接想出来方程就直接动态规划，不行就先暴力回溯然后找好状态加答案记录表写记忆化搜索。另外再注意一下有时候从右往左比从左往右写起来方便，不过这两种复杂度都一样，写哪个方向都可。|
|LC91 Medium|字符串拆分问题，分类讨论，那就是当第i个字符加入的时候，既可能作为单字符又可能作为双字符解析，判断条件满足一个累加一个即可。另外注意字符串中的递归式 f(i) 的 i 是第几个字符，因为 0 要作为无字符情况是边界条件，这个边界值要手动赋值，这里初始是 f(0) 为 1，因为无字符也是一种解析，而不是零，所以 f(1) 对应的是 str\[0\] 第一个元素，这个写的时候要注意，字符串下标要减一。|
|LC816 Medium|暴力枚举，循环拆字符串成两部分，先枚举 ',' 左右的字符串，再内部分别枚举左右字符串无小数点和有小数点(同样是循环拆开字符串)的情况，注意处理前导 0，小数后面 0 如 0.10 等恶心的格式问题。|

#### 最长递增子序列

|题号|笔记|
|:-|:-|
|LC300 Medium||
|LC376 Medium||
|LC646 Medium||
|LC673 Medium||

#### 最长公共子序列

|题号|笔记|
|:-|:-|
|LC1143 Medium|、|

#### 数值拆分问题

|题号|笔记|
|:-|:-|
|LC279 Medium||
|LC343 Medium||
|LC279 Medium|f(i) 表示凑成整数 i 所需的完全平方数的最少数量，f(i) = min{ f(i - e) + 1 | e∈nums }，比 322 题就是多了一个求有几种完全平方数的步骤。|
|LC322 Medium|零钱兑换，也是凑数问题，问用其他价格的硬币为了凑出目标值，最小需要多少枚。这道题看起来用暴力搜索做很容易，但是这样会超时，所以还是要用DP的思路。f(i) 定义成凑成 i 块钱所需要的最小硬币数，f(i) = min(全部币值情况的f(i - coin)) + 1。|

#### 背包问题

|题号|笔记|
|:-|:-|
|||

#### 树形DP

|题号|笔记|
|:-|:-|
|LC310 Medium|to do|

### **贪心思想**

贪心要需要证明该问题在局部情况不断取最优解，到了最后一定得到全局最优解，这样我们过程中只要不断取最优解即可（比如常见的局部最优解如给满足中的最差的，不断取左右两端最值，最大最小值等情况）。

水题：LC860 Easy、LC976 Easy

|题号|笔记|
|:-|:-|
|LC53 Easy|最大连续子数组和，利用 sum 记录总和，如果是正那么要保留，因为之后有了这个会更大，如果是负那么直接舍弃然后重新计数，因为后面不可能需要这个。|
|LC455 Easy|要明白为了尽可能给更多人饼干，要给每个人尽量小的，另外要注意排序胃口，这样小胃口满足不了后面大胃口更满足不了，直接终止循环。|
|LC561 Easy|贪心思想，为了避免短板发现两个数尽量接近的情况下结果最大。|
|LC605 Easy|贪心思想，能种花就种花即可，另外注意边界情况，可以在左右添加 0 来避免分类讨论。|
|LC942 Easy|遇到 < 左侧放最小值，遇到 > 左侧放最大值即可，因为这样自动保证右侧无论放哪个剩余值都满足。|
  
### **二分思想**

prob 153 34

注意二分查找的前提是向量已排好序，所以如果题目告诉你已排好序(或者你需要手动排序)，然后查找某一个元素或者重复元素的最左、最右侧的元素，可以使用二分查找。

二分思想在于通过比较 nums\[mid\] 得到要查询的子范围(根据不同情况，改变左右边界，另外要注意子范围包不包含 mid 下标)，从而再去查这个子范围，随着范围越来越小，最后会收敛在一个下标，即为答案。

水题：LC278 Easy、LC441 Easy

|题号|笔记|
|:-|:-|
|LC69 Easy|平方根不要再用循环一个一个试了，这个复杂度是O(根号X)；而使用二分则是O(logX)，快非常多。|
|LC74 Medium||
|LC378 Medium||
|LC540 Medium|二分查找，利用奇偶性发现数值对应的规律。为了避免溢出 mid 要写成这种形式 (high - low) / 2 + low。|
|LC744 Easy|利用二分查找，由于是查找第一个比目标大的，所以小于等于目标都不符合范围要 l = mid + 1，而大于的可能是答案也可能在右侧，所以 r = mid，收敛到 l == r 时注意可能是没有答案要分类讨论，有答案就是 nums\[l\]。|

### **分而治之**

把一个复杂的问题分成两个或更多的相同或相似的子问题，再递归地把子问题分成更小的子问题，直到最后子问题可以简单的直接求解，最后原问题的解可以由子问题的解合并得到。

水题：

|题号|笔记|
|:-|:-|
|LC95 Medium||
|LC241 Medium||

### **数学问题**

#### 简单的数学问题

水题：LC263 Easy

#### 数值转换

|题号|笔记|
|:-|:-|
|LC7 Medium|因为不让用超过32位，通过数学推导，我们可以推出 ret > INT32_MAX / 10 || ret < INT32_MIN / 10 的下一步会溢出[推导原理链接](https://leetcode-cn.com/problems/reverse-integer/solution/zheng-shu-fan-zhuan-by-leetcode-solution-bccn/)|
|LC8 Medium|看起来简单，但是要处理前导 0，正负值，数值越界，非法字符(比较是否 ch >= '0' && ch <= '9'即可>)等情况。|
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

常用于处理连续字串、连续数组的问题，分为固定窗口大小和可变窗口大小两类。其中固定窗口的题很多要么题干告诉你，要么两个字符串之类的其中一个小字符串的长度。

如果题目中发现有说连续的子串、子数组，只要有连续二字，一定要思考是不是滑动窗口的题型，然后一般题型是求最值的问题，然后条件一些很典型的比如告诉你可以 k 次更改/反转/忽略某一个元素，这种题你就需要一个变量，用于记录当前窗口内已经调整元素的次数，然后内层循环作为条件与 k 比较。

对于可变窗口，发现了一个模型，问题一般是求最值，开始初始化 l 和 r 为 0，一个 sz 和一个记录条件的变量，外层循环不断增大 r 作为新加入窗口的元素，然后如果条件不满足那么内层循环不断增大 l 使得条件再次满足，之后当内层循环结束即条件满足的情况，更新最值即可，下面代码是随便举的一个例子(LC1208)。

```C++
class Solution {
public:
    int equalSubstring(string s, string t, int maxCost) {
        int l = 0, r = 0;
        int sz = s.size();
        int ret = INT32_MIN;
        int cost = 0;

        while(r < sz){
            cost += abs(t[r] - s[r]);
            while(cost > maxCost){
                cost -= abs(t[l] - s[l]);
                ++l;
            }
            ret = max(ret, r - l + 1);
            ++r;
        }

        return ret;
    }
};
```

注意另一种题型问的是符合条件的个数而不是求最值，(举例LC1248，见题解)，那么我们需要 to do

另外一种常见的题型是，让你不断选取/扔掉向量的最左或最右元素 k 次，求选取的/剩余的部分的最值，这个很容易地发现这是个固定窗口问题，size - k 就是连续子数组的固定窗口大小，然后求最值即可。另外注意选取部分的最值可以通过总和减去滑动窗口的最值来得到，需要先遍历求出元素总和多转换一步而已(LC1423，见题解)。

prob 76 424 480 930 932 978 992 1610 1838

水题：LC187 Medium、LC487 Medium、LC1208 Medium

|题号|笔记|
|:-|:-|
|LC3 Medium|通过 unordered_map 记录窗口内的元素，然后如果没有重复的话就右移 r，不断找最大值，直到重复，此时不断右移 l 直到不再重复。本质上来说遇到重复元素时肯定之前已经经过上一个最长连续不重复子串并记录下最大值了，所以正确性没问题。|
|LC209 Medium|可变窗口的典型题，记录 l 到 r 范围内的总和 sum，不满足时 r 一直右移，这个写成外层 for 循环(或者 while 更容易理解)，直到满足时为了尽可能小，内部再循环把 l 尽可能左移，找到最小值。|
|LC220 Medium|to do         最简单的暴力遍历复杂度 O(nk) 会超时。|
|LC239 Hard|容易想的方法是循环+堆排序，复杂度 O(n logn)，注意堆里面用 pair<int, int> 存下标，堆里面存着超出范围的元素没有问题，只要我们取出的时候循环判断一下是不是再窗口内即可，不在的话舍弃并再取，直到取出一个在范围内的，那就是这个窗口内的最大值。另一种方法是单调队列的解法，见[此链接](#单调队列)。难忘的一题，智臾一面出的题，然而我面完第二天就刷到了这个题，要是早一天就好了。。。|
|LC396 Medium||
|LC438 Medium|这道题简单，因为窗口的长度是固定的，是 p 的长度，我们只需要不断右移整个窗口，并对比这26个字母的个数是否相同即可。可以使用 unordered_map 来比较(直接用 == 运算符即可，比较 key、key 和其对应的 value 是否全部相同，和顺序无关))，但一定要注意，map\[index\] 值为0和不存在这个 key 的元素是不一样的；也可以用vector来比较，下标对应字母即可。|
|LC567 Medium|对于判断单词是否异构(即每个字符的个数相同)这种形式，我们可以直接用 unordered_map 来实现，但是注意没插入过字母和加入字母又删除字母导致值为 0 是两种不同的概念，这里需要注意当为 0 要 erase 对应的键值对。当然更简便和推荐的方式是直接使用 vector<int> m(26)，这样就不用区分 0 还是不存在的情况了。|
|LC643 Easy|水题，给了 k，是固定窗口，一开始先计算前k个元素来初始化，然后从下标 k 开始循环 sum = sum - nums\[i - k\] + nums\[i\] 即可。|
|LC904 Medium|典型的可变窗口题型，记录 l 和 r，外侧循环 r 不断右移，并不断比较最大值，直到不满足内部循环 l 不断右移扔掉最左侧的值，来让窗口符合要求。|
|LC1004 Medium|可变窗口，另外需要记录窗口内 0 的个数，如果小于等于 k，那么不断外层循环右移 r 即可，如果大于 k，那么内层循环要不断右移 l，直到满足条件。|
|LC1052 Medium|发现最终结果应该是不生气时的顾客总数，加上原本生气现在忍住的顾客数，题干告诉强忍时间是必须连续的，所以这道题是个固定滑动窗口大小的题，伴随忍住不生气的窗口移动即可。|
|LC1234 Medium|to do|
|LC1248 Medium|to do 注意这道题和别的滑动窗口题不一样，它问的是符合条件的个数，而不是求最值之类的问题，导致我们无法简单的套用模板做出。|
|LC1423 Medium|不断选取向量的最左或最右元素 k 次，求选取的部分的最值，这个很容易地发现这是个固定窗口问题，size - k 就是连固定窗口大小，然后可以得到滑动窗口的最值。之后选取部分的最值通过总和减去滑动窗口的最值来得到，需要先遍历求出元素总和多转换一步而已。|
|LC1438 Medium|这道题框架仍然是最典型的可变窗口，但难点在于如何快速得到窗口内最大值与最小值的差值，如果能得到这个差值，那么这道题就是一个送分题。我一开始写的是用两个堆，一个最大堆一个最小堆，里面元素是 pair<int, int> 第二个 int  记录下标，然后我们明白就算是超出窗口的元素放到堆里面也是无妨的，只要我们取最大/最小时额外判断下 second >= l ，如果小于则 pop 即可，保证我们得到的确实是窗口内的最大和最小值，然后利用这两个值做差得到，但这个写法无疑有些麻烦。我们需要元素有序且可以重复，这样我们就可以用最后一个元素减去第一个元素即可得到差值，然后当要右移 l 时我们还需要能从容器中删除那个特定的元素，vector 可以但是太过麻烦，我们可以用 multiset 这个有序可重复集合，通过 begin 和 rbegin 获取第一个元素和最后一个元素，erase 删除时需要迭代器，那么我们就 find(value) 返回迭代器来让 erase 删除即可。这道题也可以用单调队列做，见[此链接](#单调队列)。|
|LC1658 Medium|和 LC1423 同类型题目。一开始以为是个很简单写的回溯，但发现复杂度太高。发现扔掉最左或最右端的元素，最后得到的中间子序列一定是连续的，然后又发现可以先遍历求和，转化成判断窗口内和的值，这个题就变成了典型的求连续子数组的最大和，最后滑动窗口复杂度 O(n)。|
|LC2024 Medium|典型的可变窗口题，但比较不同的是这道题我们无法判断最佳情况是 'T' 还是 'F'，因此我们分类讨论，把滑动窗口的代码提取出来作为一个辅助函数，主函数分别调用这个辅助函数计算 'T' 和 'F' 的情况，然后取最大值即可。|

### **输入输出格式**

1. 目前用到的笔试是牛客网的在线笔试，https://www.nowcoder.com/test/27976983/summary#question 这个网站可以进行输入输出格式练习。
2. 获取一行用getline(输入流，字符串引用)（默认分割符为'\n'），然后通过stringstream字符流来输出分隔空格，如果要分隔的是别的符号比如','，那么就要再次getline(stringstream流，字符串引用，分割符)来分隔，因为c++标准库里面没有split，所以要通过这种方式来实现。
3. 注意审题，是多组数据还是一组数据，有时候题干是多组数据但示例给的只有一组，让人误解。
4. 注意审题数据范围，INT32_MAX 是 2^31-1 在 10^9 ~ 10^10 范围，别忘了改用 long，如果特意说是32位机器，long 和 int同大小要用 long long。
5. 另外是输出末尾不让有空格之类的问题，一般我们循环外输出第一个元素（别忘了额外判断是否为空），之后循环中输出空格加剩下元素即可。
6. 注意使用英文字符。
7. cin.ignore(n,终止字符) 函数作用是跳过输入流中n个字符，或在遇到指定的终止字符时提前结束(含终止字符)。cin 和 getline 混用，要注意 cin 按类型输入完会留下'\n'，所以我们调用一个 cin.ignore() 即可，默认参数是 n=1 忽略一个 '\n' 即可。