0043 验证错误处理的情况
![](../images/2021-07-08-11-19-13.png)
![](../images/2021-07-08-11-20-29.png)
如图，20行添加了attribute，所以panic了反而通过。

0206 让should panic更精确
![](../images/2021-07-08-11-22-20.png)
有时候因为其它错误也会panic，但不是我们想要的错误，所以需要让它更精确。
![](../images/2021-07-08-11-24-01.png)
![](../images/2021-07-08-11-24-21.png)