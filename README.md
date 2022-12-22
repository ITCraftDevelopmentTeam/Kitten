# Kitten
Modified from [KittenVM](https://github.com/Shirasawa-CN/KittenVM), but is a separate RT.  

[Simplified Chinese?](./README_cn.md)  

## TODO
1.IO  
2.JIT  
3. Monitor process  
4. Garbage collection process  

## FEATURES
This toy has a lot of weird and wonderful features, please move to [README](. /docs/README.md)

## Use
## Create space
```
new name
```
Create a memory space named name

## Move data
```
mov name,4
```
Move the number 4 to the name space

```
mov name1,name2
```
Move the value of name2 to name1, then name2 becomes None

## Memory recovery
```
add_gc a
```
Add a to the recycle list
```
free
```
Clear the memory in the recycle list

## Operations
add and div mul or sll sra sud xor The format of each of these instructions is as follows

```
expr rs1,rs2,target
```