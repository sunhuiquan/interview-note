#include <iostream>
using namespace std;

class A
{
public:
	virtual void f()
	{
		cout << "a" << endl;
	}
};

class B : public A
{
public:
	void f()
	{
		cout << "b" << endl;
	}
};

int main()
{
	A a;
	B b;

	A *p1 = &a;
	cout << typeid(p1).name() << endl;
	A *p2 = &b;
	cout << typeid(p2).name() << endl;

	p1->f();
	p2->f();

	if (typeid(p1) == typeid(p2))
		cout << "true" << endl;
	else
		cout << "false" << endl;

	return 0;
}