#### cargo管理的项目结构：
+ src文件-存放代码主体
+ .toml文件等配置文件

#### cargo build编译后目录
+ target目录-存放cargo编译后的文件

+ cargo new：创建新的cargo项目
+ cargo build：编译生成target目录 (--release 发布构建，会进行优化编译项目，在target下多生成一个release目录来存放)
+ cargo run：编译并直接运行生成的可执行文件
+ cargo check：用来检查代码是否可以进行编译，并不产生可执行文件

