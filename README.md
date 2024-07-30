# proj27 在RT-Thread Smart上支持rust语言编写的用户态程序

| 属性     | 值                   |
| -------- | -------------------- |
| 团队成员 | 熊嘉晟、毛灿、林观韬 |
| 指导老师 | 苏曙光               |
| 参赛学校 | 华中科技大学         |

说明：本项目采用 [Apache-2.0](https://opensource.org/licenses/Apache-2.0) 开源协议

开源协议文本为[LICENSE.txt](https://gitlab.eduxiji.net/T202410487992548/project2210132-226009/-/blob/main/LICENSE.txt)

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

## 阶段划分

此处仅对每个开发阶段的工作做简单的描述，详细具体的流程和工作内容可见每周的开发日志[worklog](https://gitlab.eduxiji.net/T202410487992548/project2210132-226009/-/tree/main/worklog)

### 初赛阶段

初赛阶段的时间为3月10日到5月31日，完成的主要工作如下：

1. 制作整合外部函数接口库libc，使得用户能在Rust语言中调用musl-libc下的C库函数和RT-Thread SDK提供的API
2. 为Rust编译器添加一个到aarch64-unknown-rtsmart的编译目标，使得编译器能将Rust语言编写的代码编译成在aarch64架构上的rt-smart操作系统上运行的应用程序
3. 实现Rust标准库中常用的库，如标准输入输出库stdin和stdout，方便的创建并使用线程的thread库，用于保护线程间的共享资源的mutex库，方便的创建并使用文件的fs库等
4. 编写了方便的将用户编写的main函数转换为以 C ABI 调用约定为基础的 `main` 函数，作为程序的入口点的属性宏marco_main

### 决赛第一阶段

决赛第一阶段的时间为6月16日到7月31日，完成的主要工作如下：

1. 将原先标准输出库stdout中简单且不规范的日志输出功能抽取出来，编写了一个新的logging库，用于日志的输出，支持5个等级的日志级别用于表示消息的重要性和详细程度。
2. 修改原先的属性宏marco_main，使其支持从命令行读入用户输入的参数，便于用户与应用程序进行交互。
3. 编写完善线程通信相关的标准库，如支持通过信号量实现线程同步的标准库semaphore库，能够在线程间传递任意线程安全类型的对象数据的channel库，以支持更方便的线程通信。
4. 尝试将项目移植到真机上进行测试，使用树梅派开发板安装rt-smart环境，然后将当前项目编译出的Rust应用程序在开发板上跑进行测试。
5. 将当前项目编译出的Rust应用程序与aarch64-linux-musleabi-gcc编译器编译出的C程序进行性能对比分析。

## 项目目录结构

代码和文档都存储Gitlab仓库中。以下是仓库目录和文件描述： 

```plaintext
.
├── 技术文档.md # 项目技术文档
├── 技术文档.pdf # 项目技术文档PDF版本
├── bench # C与Rust性能对比测试程序
│   ├── c_bench
│   │   └── main.c
│   └── rust_bench
│       ├── Cargo.lock
│       ├── Cargo.toml
│       └── src
│           └── main.rs
├── data.csv # C与Rust性能对比测试结果
├── examples # 标准库测试程序
│   ├── example1 # libc和rust编译目标的测试，最小的Rust语言程序
│   │   └── hello
│   │       ├── Cargo.lock
│   │       ├── Cargo.toml
│   │       └── src
│   │           └── main.rs
│   ├── example2 # 测试marco_main属性宏和stdout库的测试程序
│   │   └── hello
│   │       ├── Cargo.lock
│   │       ├── Cargo.toml
│   │       └── src
│   │           └── main.rs
│   ├── example3 # 测试thread库的测试程序
│   │   └── thread_test
│   │       ├── Cargo.lock
│   │       ├── Cargo.toml
│   │       └── src
│   │           └── main.rs
│   ├── example4 # 测试mutex库的测试程序
│   │   └── mutex_test
│   │       ├── Cargo.lock
│   │       ├── Cargo.toml
│   │       └── src
│   │           └── main.rs
│   ├── example5 # 测试stdin库的测试程序
│   │   └── read_test
│   │       ├── Cargo.lock
│   │       ├── Cargo.toml
│   │       └── src
│   │           └── main.rs
│   ├── example6 # 测试fs库的测试程序
│   │   └── file_test
│   │       ├── Cargo.lock
│   │       ├── Cargo.toml
│   │       └── src
│   │           └── main.rs
│   ├── example7 # 测试logging库的测试程序
│   │   └── log_test
│   │       ├── Cargo.lock
│   │       ├── Cargo.toml
│   │       └── src
│   │           └── main.rs
│   ├── example8 # 测试marco_main属性宏接受参数的测试程序
│   │   └── param_test
│   │       ├── Cargo.lock
│   │       ├── Cargo.toml
│   │       └── src
│   │           └── main.rs
│   ├── example9 # 测试semaphore库的测试程序
│   │   └── semaphore_test
│   │       ├── Cargo.lock
│   │       ├── Cargo.toml
│   │       └── src
│   │           └── main.rs
│   └── example10 # 测试channel库的测试程序
│       └── channel_test
│           ├── Cargo.lock
│           ├── Cargo.toml
│           └── src
│               └── main.rs
├── img # 技术文档图片
├── libc # Rust在不同系统平台的外部函数接口
│   ├── build.rs
│   ├── Cargo.toml
│   ├── CONTRIBUTING.md
│   ├── LICENSE-APACHE
│   ├── LICENSE-MIT
│   ├── README.md
│   ├── rustfmt.toml
│   ├── src
│   │   ├── fixed_width_ints.rs
│   │   ├── fuchsia
│   │   ├── hermit
│   │   ├── lib.rs
│   │   ├── macros.rs
│   │   ├── rtsmart # Rust在rt-smart系统平台的外部函数接口
│   │   │   ├── aarch64.rs
│   │   │   ├── arm.rs
│   │   │   └── mod.rs
│   │   ├── sgx.rs
│   │   ├── solid
│   │   ├── switch.rs
│   │   ├── teeos
│   │   ├── unix
│   │   ├── vxworks
│   │   ├── wasi.rs
│   │   ├── windows
│   │   └── xous.rs
│   └── triagebot.toml
├── LICENSE.txt # 开源协议文本
├── marco_main # 将Rust风格的main函数转换为C风格的main函数的属性宏
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── README.md # 项目说明文件
├── rtsmart-std # 基于libc开发的rtsmart平台上的标准库
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       ├── api # 封装调用libc的api
│       │   ├── mem.rs
│       │   ├── mod.rs
│       │   ├── mpsc.rs
│       │   ├── mutex.rs
│       │   ├── semaphore.rs
│       │   └── thread.rs
│       ├── fs.rs # fs库
│       ├── lib.rs # 模块声明
│       ├── logging.rs # logging库
│       ├── malloc.rs # 内存分配器
│       ├── mpsc.rs # channel库
│       ├── mutex.rs # mutex库
│       ├── param.rs # marco_main接受参数封装类型
│       ├── prelude # 预引入声明
│       │   ├── mod.rs
│       │   └── no_std.rs
│       ├── puts.rs # stdout库的辅助模块
│       ├── semaphore.rs # semaphore库
│       ├── stdin.rs # stdin库
│       ├── stdout.rs # stdout库
│       ├── thread.rs # thread库
│       └── time.rs # time库
├── rust # 为添加编译目标，在rust编译器源代码中修改或添加的文件
└── worklog # 每周的工作日志
    ├── img # 每周的工作日志所使用的图片
    ├── markdown # 每周的工作日志markdown格式
    └── pdf # 每周的工作日志pdf格式
```

- 该项目完整的开发流程记录在worklog目录下的每周的工作日志内：[worklog](https://gitlab.eduxiji.net/T202410487992548/project2210132-226009/-/tree/main/worklog)
- 该项目的技术文档位于主目录下，其内包含项目的的基本情况以及技术的运用：[技术文档](https://gitlab.eduxiji.net/T202410487992548/project2210132-226009/-/blob/main/%E6%8A%80%E6%9C%AF%E6%96%87%E6%A1%A3.pdf)
- 所有文档均提供了markdown格式与pdf格式各一份
- 由于Rust编译器源代码过于庞大，因此该项目中Rust编译器部分按照编译器源代码的目录格式，仅包含修改过的文件或新增加的文件
- 测试程序的演示视频可在百度网盘中查看：链接: https://pan.baidu.com/s/1fbUtcwM0rAazOONh09krGA?pwd=1234 提取码: 1234

## 项目意义

1. Rust语言以其独特的内存管理机制和强大的编译期检查机制著称，这有助于避免常见的内存错误（如空指针解引用、缓冲区溢出等）。在嵌入式系统中，安全性和可靠性尤为重要，因为这些系统通常运行在资源受限和高要求的环境中。通过支持Rust语言，可以显著提升RT-Thread Smart的系统安全性和可靠性，减少运行时错误和潜在漏洞。
2. 随着嵌入式生态的发展，不同项目对编程语言有不同的需求。增加Rust语言支持，可以为开发者提供更多的选择，满足不同项目的特定需求。Rust语言的现代特性（如模式匹配、闭包、高级类型系统等）可以帮助开发者更高效地编写和维护代码，提高开发效率和代码质量。
3. Rust语言在近几年获得了广泛的关注和快速的发展，拥有活跃的开源社区和不断增长的开发者群体。通过支持Rust语言，RT-Thread Smart可以吸引更多的Rust开发者加入其生态系统，贡献代码、发现和修复问题，促进整个项目的健康发展。
4. Rust语言在设计上注重性能和资源利用效率，非常适合嵌入式系统。Rust的零成本抽象和高效的并发支持，可以帮助开发者编写高性能的应用程序，充分发挥嵌入式平台的硬件潜力。通过在RT-Thread Smart上支持Rust语言，可以帮助开发者创建更加高效的应用程序，提升系统整体性能。
5. Rust语言具有出色的跨平台能力，其标准库和生态系统支持多种平台。通过在RT-Thread Smart上支持Rust，可以促进跨平台开发，使开发者能够更方便地将应用程序移植到不同的嵌入式平台上，减少重复工作，提高开发效率。

## 比赛题目分析和相关资料调研

见[技术文档](https://gitlab.eduxiji.net/T202410487992548/project2210132-226009/-/blob/main/%E6%8A%80%E6%9C%AF%E6%96%87%E6%A1%A3.pdf)

## 系统框架设计

见[技术文档](https://gitlab.eduxiji.net/T202410487992548/project2210132-226009/-/blob/main/%E6%8A%80%E6%9C%AF%E6%96%87%E6%A1%A3.pdf)

## 开发计划

见[worklog](https://gitlab.eduxiji.net/T202410487992548/project2210132-226009/-/tree/main/worklog)

## 比赛过程中的重要进展

见[worklog](https://gitlab.eduxiji.net/T202410487992548/project2210132-226009/-/tree/main/worklog)

## 系统测试情况

见[技术文档](https://gitlab.eduxiji.net/T202410487992548/project2210132-226009/-/blob/main/%E6%8A%80%E6%9C%AF%E6%96%87%E6%A1%A3.pdf)

## 遇到的主要问题和解决方法

开发过程中遇到的问题主要有：

### 问题1：Rust编译器在编译目标方面的资料收集

由于Rust编译器的编译目标的资料较少，因此在为rt-smart添加编译目标时，需要查阅大量资料，包括Rust编译器的源代码，以及Rust编译器的文档。除此之外，还需查阅rt-smart和aarch64平台的相关资料，了解代码编译的目标平台的相关信息，添加到Rust编译器的编译目标中。

解决方法：查阅Rust编译器的源代码，了解Rust编译器的编译目标的相关信息，同时查阅rt-smart和aarch64平台的相关资料，了解代码编译的目标平台的相关信息，将这些信息添加到Rust编译器的编译目标中。

### 问题2：无标准库情况下编译Rust程序

无标准库的情况下编译Rust程序，无法使用Rust的main函数，同时在使用`cargo build`命令构建时，会报错找不到对应平台的标准库。

解决方法：编写属性宏将Rust风格的main函数转换为C风格的main函数，同时使用`cargo xbuild`命令，只将`alloc`库和`core`库链接到程序中，不链接标准库。

### 问题3：编写thread库和mutex库时，总会出现地址访问错误的问题

在编写thread库和mutex库时，总会出现地址访问错误的问题，导致程序无法正常运行。由于thread库和mutex库是基于libc库的封装，用到的一些结构和指针传递相对复杂，而在库函数中需要频繁进行Rust风格的指针和C风格的指针的转换，因此容易出现地址访问错误。

解决方法：在编写库函数时，反复检查指针的传递和转换，确保指针的传递和转换正确，同时在调用库函数时，确保传递的参数正确。

## 分工和协作

| 成员   | 分工情况                                                     |
| ------ | ------------------------------------------------------------ |
| 苏曙光 | 选择赛题，设计大致框架                                       |
| 熊嘉晟 | 完成框架设计、编写libc、给Rust编译器新增编译目标、编写标准库、编写技术文档和工作日志 |
| 毛灿   | 编写标准库、编写技术文档和工作日志、辅助完成细节修改         |
| 林观韬 | 编写标准库、编写技术文档和工作日志、辅助完成细节修改         |

## 提交仓库目录和文件描述

见[项目目录结构](#项目目录结构)

## 比赛收获

在参赛的过程中，我们一方面加深了对Rust语言的理解，比如`rust_target`、`core`库这些底层的建筑，或是声明宏、属性宏、生命周期、闭包等高级用法。另一方面，我们对Unix环境编程也有了更多的实践经验，例如我们在编写libc和标准库的过程中多次使用到了musl-libc中的库函数，让我们对Unix环境下这些系统库函数的使用更加熟练。除此之外，我们对rt-thread环境下的编程也了解了许多，比如线程和互斥量的使用，系统的一些底层功能等等。

在完成项目的过程中，我们查阅了许多资料，也遇到了很多难以解决的问题，比如地址访问错误，线程死锁，线程冲突等难以查明的错误，网上与本题相关的信息极少，只能通过自己的探索一步一步达成我们的目标，找出错误的原因。在这个过程中，也锻炼了我们定位bug，修复bug的能力。

更加重要的是，我们学习到了一个新的系统平台与一门新的编程语言是如何进行沟通的，通过我们的项目，成功使得Rust语言在`aarch64-unknown-rtsmart` 平台上使用成为可能，我们非常荣幸能够成为Rust语言在嵌入式平台上的生态的贡献者。

最后，我们要感谢老师在这个学期对我们的指导和帮助，让我们能够顺利完成这个项目，也感谢我们的队友在这个项目中的辛勤付出，我们在完成项目的过程中培养了团队合作和沟通的能力，也学到了很多新的知识，这对我们的学习和成长都是非常有益的。

## 其他参考项目

1. https://gitlab.eduxiji.net/33461599/libc-for-rust-rt-samrt

   本项目引用并参考了该项目rt-smart操作系统的外部函数接口库。

   区别与创新点：本项目引入了更多的RT-Thread SDK提供的API函数，以支持更多的系统功能，如线程、信号量、消息队列等相关接口。

2. https://github.com/rust-for-rtthread/rtt_rust

   本项目借鉴了该项目标准库的设计模式，与上一项目结合，通过调用外部函数接口库，编写rt-smart操作系统使用的Rust标准库。同时也借鉴了该项目通过过程宏的方式对用户编写的代码进行重构，将用户的main函数转变为C ABI调用格式的main函数作为程序的入口点的思路，设计并实现了marco_main这一过程宏。

   区别和创新点：本项目实现了更多的标准库，如文件操作相关的fs库，并且将消息队列设计成了更符合Rust语言标准库规范的channel，使得使用更加方便，且与官方的设计统一。