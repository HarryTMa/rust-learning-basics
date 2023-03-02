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

### minesweeper
这个更烦，最主要它还建议不要clone原字符串，那我就只能一个字符一个字符push了。`isize usize`搞了半天。

### Parallel Letter Frequency
啊啊，原来这个网站是可以直接做的，我才发现
这个更烦。单独`thread::spawn`不能在闭包传引用，要用`thread::scope`绕个圈子；
它的题目描述非常晦涩，但是我最后发现它要的是“letter”，所以要包含任何文字的字母，但不能包含数字或符号。虽然很离谱，但是可以用`c.is_alphabetical()`判断。

### 自选
### Poker(https://exercism.org/tracks/rust/exercises/poker)，难度Hard
之所以选这个是因为感觉比较好玩，点进去它给的参考资料，发现这种游戏和大怪路子非常相近（果然牌类游戏都是泊来品）。有一个坑是A2345算顺子，还有一个坑是10它的字符串就是"10"，不是"T"。总体不难，虽然我做了两个小时。我又不得不提到Rust的坑点：不同类型的整数甚至不能运算，甚至不能隐式转换。真的太坑了。反思一下，我觉得我代码写得也不好，无所谓了。

### Clock(https://exercism.org/tracks/rust/exercises/clock)，难度Medium
这个更简单了，感觉这个网站上的Medium相当于一般的Easy。我唯一不会的是格式化输出整数，但是查阅了资料就会了。

### Grade School(https://exercism.org/tracks/rust/exercises)，难度Medium
这个就是一些迭代器和二叉树容器的应用，不再赘述。

### Luhn(https://exercism.org/tracks/rust/exercises/luhn)，难度Medium
这个唯一值得多说的是，我的算法可能不是特别好，用了额外的空间，事实上可以不用的。

# 其他感受
有一个很大很大的坑，就是在任务二里面我一开始忘了在`mod.rs`里面添加`pub mod xxx`，然后编译没有问题，我心说我也没那么厉害吧，然后一放到网站上去，几十个错误。乐。
然后是我知道怎么让cargo不显示警告。因为我定义了太多「没用」的函数和结构，所以cargo报了一堆警告，所以想在其中找到错误就有了难度。不过我也查不到怎么让它用类似`gcc -Wno-warnings`的命令行参数，遂作罢。
