# proj27 在RT-Thread Smart上支持rust语言编写的用户态程序

| 属性     | 值                   |
| -------- | -------------------- |
| 团队成员 | 熊嘉晟、毛灿、林观韬 |
| 指导老师 | 苏曙光               |
| 参赛学校 | 华中科技大学         |

## 目标描述

RT-Thread Smart（简称rt-smart）是适用于嵌入式平台的实时操作系统。

目前RT-Thread Smart的用户态应用程序只支持C/C++程序，它使用musl libc。而随着嵌入式生态的发展，我们认为在rt-smart上运行Rust语言的应用程序也是相当重要的。本项目旨在为rt-smart增加Rust语言用户程序支持。

## 预期目标

**注意：下面的内容是建议内容，不要求必须全部完成。选择本项目的同学也可与导师联系，提出自己的新想法，如导师认可，可加入预期目标**

### 第一题：Rust语言支持库(已完成)

- 编译并运行rt-smart操作系统。编译一个最小的Rust语言程序，使之能在rt-smart操作系统上运行；
- 查阅rt-smart的文档和资料，了解其中可用的系统调用。查阅rt-smart与编译器有关的文档，了解rt-smart遵守的调用约定；
- 选择一个系统调用的模块，制作一个封装和处理这部分系统调用的库。依赖于这个库，用户可以编写应用于rt-smart操作系统的应用程序，并使用里面的系统调用。请参考libtock-rs。

### 第二题：制作一个到rt-smart的Rust编译目标(已完成)

- 查阅Rust语言相关的资料，了解Rust语言目前支持哪些平台。Rust语言是否拥有面向嵌入式操作系统专门的编译目标？请举例；
- 作为一个独立的平台，rt-smart提供平台相关的开发方法。请为Rust编译器添加一个名为“aarch64-rt-smart”的编译目标。使用该目标得到的二进制程序，能直接应用到rt-smart支持的平台上；
- 请将第一题的成果视作标准库的一部分，选择一个系统调用的模块，为aarch64-rt-smart添加std标准库对应模块的代码。

### 第三题：Rust生态中的rt-smart平台(已完成)

- 许多的库实现和平台有关系，这些库通常需要特定的操作系统功能支持。比如num_cpus，它在不同平台上调用不同的系统调用，达到列出CPU核数量的功能。查阅资料，列出几个这样的库。[您可能需要浏览crates.io或lib.rs](http://xn--crates-2b6jp45f6hzoe5bfutnya307j.xn--iolib-ym6j.rs/)。
- 选择一个Rust生态中的库，为其添加代码，使之能支持rt-smart操作系统的相关功能。

### 项目目录结构

代码和文档都存储Gitlab仓库中。以下是仓库目录和文件描述：

```plaintext
.project2210132-226009/						
│   README.md				//项目说明文件
│   技术文档.md				 //技术文档
│   技术文档.pdf			 //技术文档PDF版本
├─ img/                     //技术文档图片
├─ examples/                //标准库测试程序
│	├── example1/hello		//libc和rust编译目标的测试，最小的Rust语言程序
│	├── example2/hello		//测试marco_main属性宏和stdout库的测试程序
│	├── example3/thread_test//测试thread库的测试程序
│	├── example4/mutex_test //测试mutex库的测试程序
│	├── example5/read_test  //测试stdin库的测试程序
│	└── example6/file_test  //测试fs库的测试程序
├─ libc/					//Rust在不同系统平台的外部函数接口
├─ marco_main/				//将Rust风格的main函数转换为C风格的main函数的属性宏
├─ rtsmart-std/				//基于libc开发的rtsmart平台上的标准库
│	├── src/				//源代码
│		├── api/			//封装调用libc的api
│		├── prelude/		//预引入的库
│		│   fs.rs			//fs库
│		│   lib.rs			//模块声明
│		│   malloc.rs		//内存分配器
│		│   mutex.rs		//mutex库
│		│   put.rs			//stdout库的辅助模块
│		│   stdin.rs		//stdin库
│		│   stdout.rs		//stdout库
│		│   thread.rs		//thread库
│		└── time.rs			//time库
├─ rust/					//为添加编译目标，在rust编译器源代码中修改或添加的文件
├─ video/					//标准库测试程序的演示视频
└─ worklog/					//每周的工作日志
```

- 该项目完整的开发流程记录在worklog目录下的每周的工作日志内：[worklog](https://gitlab.eduxiji.net/T202410487992548/project2210132-226009/-/tree/main/worklog)
- 该项目的技术文档位于主目录下，其内包含项目的的基本情况以及技术的运用：[技术文档](https://gitlab.eduxiji.net/T202410487992548/project2210132-226009/-/blob/main/%E6%8A%80%E6%9C%AF%E6%96%87%E6%A1%A3.pdf)
- 所有文档均提供了markdown格式与pdf格式各一份
- 由于Rust编译器源代码过于庞大，因此该项目中Rust编译器部分按照编译器源代码的目录格式，仅包含修改过的文件或新增加的文件
- 测试程序的演示视频位于video目录下：[video](https://gitlab.eduxiji.net/T202410487992548/project2210132-226009/-/tree/main/video)