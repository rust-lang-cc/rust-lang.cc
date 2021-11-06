# minigrep

本例子来自 The book 第十二章  

**前置知识**：完成 [The book](http://120.78.128.153/rustbook/) 第一 至 十一章  

## 仿写 内容搜索命令行 grep

最终完成程序的使用方法：  
minigrep search_string filename.txt  
minigrep 搜索词 文件名

运行结果：打印出指定文件中包含搜索词的行。[^1]  

<br>

--- 

[^1]: (以及可在环境变量中保存 CASE_INSENSITIVE 变量值来使搜索结果为大小写不敏感)