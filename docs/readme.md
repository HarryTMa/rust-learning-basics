# 实践过程
### part1
运行`cargo test part1_test`时，由于part2~4还没完成，所以编译就会出错，需要手动把main.rs里面的`mod part2;`等去掉。

### part2
这部分我几乎全部理解对了，只有`trim()`返回的是`&str`而不是`String`错了。现在想想也对，`trim()`只去掉首尾，没必要新开一片内存来拷贝。不过有些方法是泛型方法，比如`into()`，它可以`into::<&str>()`也可以`into::<String>()`，经尝试这里函数用`string_slice`与`string`皆可。

### part3
这部分比较简单，唯一的坑点就是指南里面的shell是`cargo test return_twice_of_positive_numbers`，但是其实方法名叫`returns_twice_of_positive_numbers`（恼）。

### part4