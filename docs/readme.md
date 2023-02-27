# 实践过程
## 任务一
第一节略
### part1
运行`cargo test part1_test`时，由于part2~4还没完成，所以编译就会出错，需要手动把main.rs里面的`mod part2;`等去掉。

### part2
这部分我几乎全部理解对了，只有`trim()`返回的是`&str`而不是`String`错了。现在想想也对，`trim()`只去掉首尾，没必要新开一片内存来拷贝。不过有些方法是泛型方法，比如`into()`，它可以`into::<&str>()`也可以`into::<String>()`，经尝试这里函数用`string_slice`与`string`皆可。

### part3
这部分比较简单，唯一的坑点就是指南里面的shell是`cargo test return_twice_of_positive_numbers`，但是其实方法名叫`returns_twice_of_positive_numbers`（恼）。

### part4
宏我还是不熟悉，用的时候要看其他项目的源码或者看手册。以及我不知道如何在`macro_rules!`定义的宏里面合并两个字面量，只能简单地用`format!`来做。（不过有一说一，编译时如果开了优化，Rust自己会帮我们合并的。）

## 任务二
### sublist
`Vec<T>`的`len()`返回一个`usize`，所以做减法的时候要转成`isize`，但访问下标要再转回`usize`，就很烦。另外，KMP我早已忘光光了，所以就直接用朴素的`O(mn)`法了。

