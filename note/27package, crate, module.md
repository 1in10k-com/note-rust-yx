0000 rust的代码组织
[](https://github.com/playdog-io/note-rust-yx/blob/main/ph/QQ%E6%88%AA%E5%9B%BE20210704183020.png)

0300 package和crate

[](https://github.com/playdog-io/note-rust-yx/blob/main/ph/QQ%E6%88%AA%E5%9B%BE20210704183415.png)

0350 cargo惯例：src/main.rs 是binary crate的crate root。较复杂，看视频。

0547 定义module来控制作用域和私有性。较复杂，看视频。

----
以上图片显示不正常，就暂时去掉! ，让图片不显示。
### 更新内容
0140
![](images/2021-07-09-16-02-11.png)
越上方包含的范围越大

---
![](images/2021-07-09-16-07-19.png)

---
![](images/2021-07-09-16-08-20.png)
package名就是项目名

---
![](images/2021-07-09-16-13-26.png)
如果有 main.rs ，就说明有个binary crate，如果有lib.rs 就说明有个library crate。  
他俩都是入口文件，都是crate的根。而crate名和package，项目名相同。

---
![](images/2021-07-09-16-15-23.png)

---
![](images/2021-07-09-16-17-01.png)

---
![](images/2021-07-09-16-21-31.png)
![](images/2021-07-09-16-23-21.png)

***
![](images/2021-07-09-16-25-15.png)
整个模块树，都被放在名为crate的隐式跟模块下。
