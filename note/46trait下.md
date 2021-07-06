```
0000 trait作为参数

0450 实现trait作为返回类型->让函数或者方法的返回类型实现某个trait。

0628 使用trait bound的例子 ？？？重点，可复习
0645 >大于号这个运算符实际上是实现了std::cmp::PartialOrd这个trait。只有类型实现了这个trait才能用大于号来比较

1004 使用trait bound有条件的实现方法
*在使用泛型类型参数的impl块上使用Trait bound，我们可以有条件的为实现了特定Trait 的类型来实现方法（文字不好理解，看视频示例 ）
*也可以为实现了其它Trait的任意类型有条件的实现某个Trait
*为满足Trait Bound的所有类型.上实现Trait叫做覆盖实现( blanket implementations)  （看视频，例子：所有实现了display的都可以实现tostring）
```