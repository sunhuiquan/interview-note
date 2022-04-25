#include <bits/stdc++.h>
using namespace std;

template <typename T>
class smart_ptr
{
private:
	T *ptr;
	int *use_count; // 通过指针完成多个 smart_ptr 共享

public:
	smart_ptr(T *p);
	smart_ptr(const smart_ptr<T> &sp);
	~smart_ptr();

	T &operator*();
	T *operator->();
	smart_ptr<T> &operator=(const smart_ptr<T> &rhs);
	T *operator+(int i);
	int operator-(const smart_ptr<T> &lhs, const smart_ptr<T> &rhs);

	int get_count();
};

template <typename T>
smart_ptr<T>::smart_ptr(T *p)
{
	ptr = p;
	use_count = new int(1);
}

template <typename T>
smart_ptr<T>::smart_ptr(const smart_ptr<T> &sp)
{
	ptr = sp.ptr;
	use_count = sp.use_count;
	++(*use_count);
}

template <typename T>
smart_ptr<T>::~smart_ptr()
{
	if (--(*use_count) == 0)
	{
		delete ptr;
		ptr = nullptr;
		delete use_count;
		use_count = nullptr;
	}
}

template <typename T>
T &smart_ptr<T>::operator*()
{
	return *ptr;
}

template <typename T>
T *smart_ptr<T>::operator->()
{
	return ptr;
}

template <typename T>
smart_ptr<T> &smart_ptr<T>::operator=(const smart_ptr<T> &rhs)
{
	++(*rhs.use_count);
	if (--(*use_count) == 0)
	{
		delete ptr;
		ptr = nullptr;
		delete use_count;
		use_count = nullptr;
	}

	ptr = rhs.ptr;
	use_count = rhs.use_count;
	return *this;
}

template <typename T>
T *smart_ptr<T>::operator+(int i)
{
	T *p = ptr + i;
	return p;
}

// template <typename T>
// int smart_ptr<T>::operator-(const smart_ptr<T> &lhs, const smart_ptr<T> &rhs)
// {
// 	return lhs.ptr - rhs.ptr;
// }

template <typename T>
int smart_ptr<T>::get_count()
{
	return *use_count;
}

int main()
{
	return 0;
}