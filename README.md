# rbind

使用PyO3实现的Python Lists

使用Rust实现的多线程安全的List，并提供了Python绑定。
List可以作为stack使用。

## 构建方法

1. 使用venv构建虚拟环境
```shell
python -m venv .venv
source .venv/bin/activate
```
2. 安装 pyo3 生成工具
```shell
pip install maturin
```
3. 使用 maturin 构建
```shell
maturin devlop 
```

## 使用例子：
见 [Example](test/test_1.py)

```shell
python3 test/test_1.py
```

output:
```shell
3
2
1
```

