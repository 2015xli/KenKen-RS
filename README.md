# Fast solver for KenKen (KenDoku) in Rust

To run it:

    cargo run --release <kenken problem file>

The package includes a few example kenken files under ./input directory. E.g.,

    cargo run --release input/input.6
    
(There are also KenKen solver in [Java](https://github.com/2015xli/KenKen) and [Java-RX](https://github.com/2015xli/KenKen-RX) versions. )
    
You can follow the example files to create your own KenKen problems. For example, for the following KenKen problem:

![A 6x6 KenKen problem](./input/input.6.png)

The input file input.6 has the following, which uses a _single_ character to represent one region, and uses operator + - * / to represent the computation. If it is a single cell region that has no computation, leave the operator blank. Multiplication operator can be either * or x .
```
K K B B B B  
a c l s s t
a c l $ # t
a d m $ # z
b d m v x z
b w w v x z

K 6 x
B 14 +
a 10 +
c 1 -
l 7 +
s 4 x
t 2 /
d 1 -
m 1 -
$ 30 x
# 5 +
b 3 -
w 3 -
v 7 +
x 6 x
z 30 x
```
Then the output will be:

```
 6  1  4  2  5  3  
 3  5  6  1  4  2  
 2  6  1  5  3  4  
 5  4  3  6  2  1  
 1  3  2  4  6  5  
 4  2  5  3  1  6 
```
