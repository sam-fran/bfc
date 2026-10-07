# bfc
A cross compiler (to c) for the laguage brainf*ck

## Usage

```bfc [input filename] -o [output filename]```

## Language
Brainf*ck is an esoteric programming langauge, meaning it isn't like other (normal) languages. It only has eight kewords.
```
< : move the memory pointer left (down)
> : move the memory pointer right (up)
+ : add one to the address of the pointer
- : subtract one from the address of the pointer
, : Put the last char typed in the address
. : Print out the address's ascii char
[ : Loop start
] : Loop end, goes to the last loop start if the address pointed at is not 0
```

## Coming Soon

1. I am aware that there are vulnerabilities in being able to access a pointer value beyond the array. In the future, there might be a compiler option to add safety for that, at the cost of a bigger file.

2. File and I/O operations, leading the path to make this self hosted

3. Self hosting. Rewriting the compiler itself in bf.