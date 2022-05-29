#include <bits/stdc++.h>
using namespace std;

class Solution
{
public:
	string discountPrices(string sentence, int discount)
	{
		string ret;

		stringstream ss(sentence);
		string word;
		bool first = true;
		while (getline(ss, word, ' '))
		{
			if (!first)
				ret += " ";
			else
				first = false;

			if (word[0] == '$')
			{
				bool is_num = true;
				int len = word.size();
				if (len == 1)
				{
					ret += word;
					continue;
				}
				for (int i = 1; i < len; ++i)
					if (word[i] < '0' || word[i] > '9')
					{
						is_num = false;
						break;
					}

				if (!is_num)
					ret += word;
				else
				{
					ret += "$";
					char chCode[20];
					sprintf(chCode, "%.2lf", stol(word.substr(1)) * (100 - discount) / 100.0);
					ret += string(chCode);
				}
			}
			else
				ret += word;
		}

		return ret;
	}
};