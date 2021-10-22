# disguise

阿里云盘目前（2021/10/22）不支持 zip、rar 类型文件分享，但是把 zip、rar 等压缩包伪装成 png 图片就可以分享了。虽然 [ImHex](https://github.com/WerWolv/ImHex) 等 16 进制编辑器可以手动修改文件头标识，但感觉还是不太方便，所以就有了这个用`rust`语言写的小工具

# 支持三种文件类型

- png （伪装类型）
- zip （还原类型）
- rar （还原类型）

# 注意事项

1. 使用前，先给文件备份，源文件丢失，作者概不负责
2. **-i -t** 两个参数是必填项，前一个指定要转换的文件，后一个指定要转换成的文件类型（**png、zip、rar**）
3. **-o** 是转换成的目标文件（可选）

# 示例

## 伪装文件类型

```console
$ disguise  -i 池田依來沙1st寫真集：pinturita.zip -t png
```

## 还原文件类型

```console
$ disguise  -i 池田依來沙1st寫真集：pinturita.png -t zip
```

# 分享文件测试

我用阿里云盘分享了「池田依來沙 1st 寫真集：pinturita.png」，你可以不限速下载 🚀
复制这段内容打开「阿里云盘」App 即可获取
链接：https://www.aliyundrive.com/s/89Pp7CzMxV2
