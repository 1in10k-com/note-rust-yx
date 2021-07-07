fn main() {
    println!("hello world!");
}

//1，把空格所在的索引位置传过去，如没找到空格就把长度返回回去
fn first_world(s: &String) -> usize {
    //2，通过依次检查string里的每个字节，看这个字节是否为空格，所以先把string类型转化为字节数组，用string上的asbytes方法。
    let bytes = s.as_bytes();
    //3，对数组进行遍历，使用数组的iter方法。for后面是一个tuple元组。iter方法为数组创建了一个迭代器，会依次返回集合中的每个数。
    //enumerate会把iter这个方法的结果进行包装，并把每个结果作为一个元组的一部分返回。
    //而元组的第一个元素就是enumerate遍历的索引。第二个元素是数组里的元素，也就是字符串里的字节，但注意它是一个引用。
    //这里实际上用到了模式匹配，因为enumerate返回的是一个元组，所以for循环 (i, &item) 这块使用了模式匹配，分别声明了两个变量，对元组进行解构。
    for (i, &item) in bytes.iter().enumerate(){

    }
}
