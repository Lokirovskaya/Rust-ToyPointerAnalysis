# Pointer Analysis By Rust

基于 Anderson 指向分析法的流敏感的指针分析求解器

输入 `input.txt` 输出 `output.txt`

## 功能

**支持 4 种指针语句**

| 语句               | 格式      |
| ----------------- | --------- |
| Reference         | `a = &b;` |
| Alias             | `a = b;`  |
| Dereference Read  | `a = *b;` |
| Dereference Write | `*a = b;` |

语句需要以分号结尾。不支持语句的复合，例如 `*a = &b;`

**支持 3 种控制流语句**

* `if {...}`
* `if {...} else {...}`
* `while {...}`

**数据流使用 Tag 输出**

在一条语句后加入 `# <Tag>`，可以输出此条语句处的数据流。文件结尾会自动加入 `# (End)` 语句

**其它**

变量不需要声明，所有变量的作用域都是全局的。输入格式错误可能导致 `panic`



## 例子

**简单的例子**

输入 `intput.txt`

```c
o = &v;
q = &p;     #1
if {
    p = *q;
    p = o;  #2
}
w_ = &w;    #3
*q = w_;
```

输出 `output.txt`

```
# 1:
  o -> [v]
  q -> [p]

# 2:
  o -> [v]
  p -> [v]
  q -> [p]

# 3:
  o -> [v]
  p -> [v]
  q -> [p]
  w_ -> [w]

# (End):
  o -> [v]
  p -> [v, w]
  q -> [p]
  w_ -> [w]
```



**if-else 流**

输入`input.txt`

```c
if {
    a = &b; #if_branch
}
else {
    a = &c; #else_branch
    x = &y;
}
```

输出 `output.txt`

```
# if_branch:
  a -> [b]

# else_branch:
  a -> [c]

# (End):
  a -> [b, c]
  x -> [y]
```



**while 流**

输入 `input.txt`

```c
_x = _a;
_a = &_b;

while {
    x = a;
    a = &b;
}
```

输出 `output.txt`

```
# (End):
  _a -> [_b]
  a -> [b]
  x -> [b]
```

