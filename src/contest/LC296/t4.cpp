#include <bits/stdc++.h>
using namespace std;

class TextEditor
{
public:
	TextEditor()
	{
		cursor_ = 0;
		len_ = 0;
	}

	void addText(string text)
	{
		text_.insert(cursor_, text);
		int sz = text.size();
		cursor_ += sz;
		len_ += sz;
	}

	int deleteText(int k)
	{
		if (cursor_ <= k)
		{
			int t = cursor_;
			text_ = text_.substr(cursor_);
			cursor_ = 0;
			len_ -= t;
			return t;
		}

		text_.erase(cursor_ - k, k);
		cursor_ -= k;
		len_ -= k;
		return k;
	}

	string cursorLeft(int k)
	{
		if (cursor_ <= k)
		{
			cursor_ = 0;
			return "";
		}

		cursor_ -= k; // abcde cursor_=3
		if (cursor_ > 10)
			return text_.substr(cursor_ - 10, 10);
		return text_.substr(0, cursor_);
	}

	string cursorRight(int k)
	{
		if (cursor_ + k > len_)
			cursor_ = len_;
		else
			cursor_ += k;

		if (cursor_ > 10)
			return text_.substr(cursor_ - 10, 10);
		return text_.substr(0, cursor_);
	}

private:
	int cursor_;
	string text_;
	int len_;
};

int main()
{
	TextEditor textEditor = TextEditor();	   // 当前 text 为 "|" 。（'|' 字符表示光标）
	textEditor.addText("leetcode");			   // 当前文本为 "leetcode|" 。
	cout << textEditor.deleteText(4) << endl;  // 返回 4
											   // 当前文本为 "leet|" 。
											   // 删除了 4 个字符。
	textEditor.addText("practice");			   // 当前文本为 "leetpractice|" 。
	cout << textEditor.cursorRight(3) << endl; // 返回 "etpractice"
											   // 当前文本为 "leetpractice|".
											   // 光标无法移动到文本以外，所以无法移动。
											   // "etpractice" 是光标左边的 10 个字符。
	cout << textEditor.cursorLeft(8) << endl;  // 返回 "leet"
											   // 当前文本为 "leet|practice" 。
											   // "leet" 是光标左边的 min(10, 4) = 4 个字符。
	cout << textEditor.deleteText(10) << endl; // 返回 4
											   // 当前文本为 "|practice" 。
											   // 只有 4 个字符被删除了。
	cout << textEditor.cursorLeft(2) << endl;  // 返回 ""
											   // 当前文本为 "|practice" 。
											   // 光标无法移动到文本以外，所以无法移动。
											   // "" 是光标左边的 min(10, 0) = 0 个字符。
	cout << textEditor.cursorRight(6) << endl; // 返回 "practi"
											   // 当前文本为 "practi|ce" 。
											   // "practi" 是光标左边的 min(10, 6) = 6 个字符。

	return 0;
}
