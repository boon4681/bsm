# BSM

Assembly but it's boon4681

## Registers

register A // general

register B // general

register C // general

register T // temporary

## Instructions

```
SET <A> <VALUE>
    Set register <A> with <VALUE>

CMP <A> <B>
    Compare value <A> with value <B>

JMPZ <LINE>
    Jump to <LINE> if T reg is zero
JMP <LINE>
    Jump to <LINE> ifT reg is not zero

ADD <A> <VALUE>
    Add <A> reg by <VALUE>

SUB <A> <VALUE>
    Subtract <A> reg by <VALUE>

MULT <A> <VALUE>
    Multiple <A> reg by <VALUE>

DIV <A> <VALUE>
    Divine <A> reg by <VALUE>

PRNT <VALUE>
    Print <VALUE> to screen
```