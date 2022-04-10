#include <iostream>
#include <string>
#include <sstream>

using namespace std;

int main()
{
	string s;
	getline(cin, s);
	stringstream ss(s);
	int a;
	char b;

	ss >> a >> b;
	cout << a << b;

	return 0;
}