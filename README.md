# Kitten
修改自[KittenVM](https://github.com/Shirasawa-CN/KittenVM),但是是一个独立的RT。


## 特性
本玩具具有许多奇奇怪怪的脑洞特性，请移步到[README](./docs/README.md)

# 使用(指令)
## 创建空间
```
new name
```
创建一个名为name的内存空间

## 移动数据
```
mov name,4
```
将数字4移动到name空间

```
mov name1,name2
```
将name2的值移动到name1，此时name2变成None

## 内存回收
```
add_gc a
```
将a添加到回收列表中
```
free
```
清理回收列表中的内存

## 运算
add and div mul or sll sra sud xor 这些运算指令的格式均如下

```
expr rs1,rs2,target
```