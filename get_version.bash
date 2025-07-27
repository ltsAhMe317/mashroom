#!/bin/bash

# 获取 git describe 信息
GIT_DESCRIBE=$(git describe --always 2>/dev/null)

# 检查是否成功获取到版本信息
if [ -z "$GIT_DESCRIBE" ]; then
    echo "Error: 无法获取 git 版本信息，当前目录可能不是 git 仓库"
    exit 1
fi

# 将版本信息写入 VERSION 文件
echo -n "$GIT_DESCRIBE" > VERSION

echo "版本信息已写入 VERSION 文件: $GIT_DESCRIBE"
