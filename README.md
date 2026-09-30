# bfc
A cross compiler (to c) for the laguage brainf*ck

## Usage
I'm not done with this right now, but the usage should be

```bfc [input file (bf)] [output file (c)]```

## Language
Brainf*ck is an esoteric programming langauge, meaning it isn't like other (normal) languages. It only has eight kewords.
```
< : move the memory pointer left (down)
> : move the memory pointer right (up)
+ : add one to the address of the pointer
- : subtract one from the address of the pointer
. : Print out the address's ascii char
, : Put the last char typed in the address
[ : Loop start
] : Loop end, goes back to the last loop start if the address pointed at is not 0
```