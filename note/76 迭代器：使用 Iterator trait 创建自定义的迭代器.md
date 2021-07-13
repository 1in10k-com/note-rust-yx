### 迭代器：使用 Iterator trait 创建自定义的迭代器

![](images/2021-07-13-14-52-34.png)
实际上只用一步，就是提供next方法的实现。
***

例子，做一个从1遍历到5的迭代器。  
![](images/2021-07-13-15-04-28.png)
![](images/2021-07-13-15-18-03.png)
![](images/2021-07-13-15-18-33.png)
zzzz  
a,count用来存储迭代过程中所需的数值，也就是迭代过程的状态。count字段是私有的，这样Counter这个结构体就能独立管理它的值了。    
b，然后做了一个关联函数，new函数，相当于是个构造函数。这样就能确保我们Counter（教程此时指的是第6行的Counter，dddd）实例里的count字段都是从0开始。  
c，接下来我们要实现这个自定义的迭代器就需要对Counter实现Iterator这个trait。前两节介绍过Iterator这个trait，他有两个关联类型，一个是type item，一个是next方法，主要是next方法。  
d，首先把type Item这个关联类型指定为u32，具体语法在第19章细讲，现在只需要知道这个迭代器会返回u32类型的数据。  
e，然后看下next方法，它的返回类型是option self::item，可以理解为就是option u32。  
f，这里就是普通的函数用法了，不做笔记。0125。所有做完后就拥有了一个自定义迭代器。测试过程就是调next方法  
以下是详细代码
```
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]

fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

fn main() {}

```

***
![](images/2021-07-13-15-22-11.png)
zzzz 这里有个更进阶的介绍，用到了zip，map，filter。0200s  

接下来扩展下，使用其它的iterator trait的方法。需求是这样的，之前的迭代器能从1迭代到5，我们想要两个这样的迭代器，它们的每对元素相乘。第一个迭代器跟之前的一样，从1到5。第二个从2开始，2，3，4，5。四个元素。最终产生的新迭代器是1*2，2*3，3*4，4*5。最终的5对应的是none。但新迭代器产生的元素有要求，要求元素必须被3整除，所以还需要过滤下。最终把剩下的元素求和并返回。  
zip，英文意思是拉链，需要两个迭代器。把两个迭代器的元素像拉链一样组合到一起，产生新的迭代器。新迭代器中每个元素都是个元组。  
a,第一个迭代器，原封不到，从1迭代到5。  
b，第二个迭代器，使用skip方法，传进去1，表示略过一个元素。  
c, map的参数是新迭代器的元素类型，所以是元组。用了模式匹配。
