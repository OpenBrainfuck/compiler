# The OpenBrainfuck Compiler

**NB** : Isn't a true compiler, but a transpiler from openbrainfuck script file or brainfuck file to C lang file.

---

## Simple usage

Here, this command transpile the source file to c-lang file.
```
obf [obf source file] -o [name of the output c program]
```

If you need help with the CLI (*Command Line Interface*), you can type this:
```
obf --help
```

## Syntax

The language consists of eight commands, listed below. A brainfuck program is a sequence of these commands, possibly interspersed with other characters (which are ignored). The commands are executed sequentially, with some exceptions: an instruction pointer begins at the first command, and each command it points to is executed, after which it normally moves forward to the next command. The program terminates when the instruction pointer moves past the last command.

The brainfuck language uses a simple machine model consisting of the program and instruction pointer, as well as a one-dimensional array of at least 30,000 byte cells initialized to zero; a movable data pointer (initialized to point to the leftmost byte of the array); and two streams of bytes for input and output (most often connected to a keyboard and a monitor respectively, and using the ASCII character encoding).

Character |	Meaning
---       | ---
`>`	      | Increment the data pointer (to point to the next cell to the right).
`<`       |	Decrement the data pointer (to point to the next cell to the left).
`+`       | Increment (increase by one) the byte at the data pointer.
`-`       |	Decrement (decrease by one) the byte at the data pointer.
`.`       |	Output the byte at the data pointer.
`,`       |	Accept one byte of input, storing its value in the byte at the data pointer.
`[`       |	If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching `]` command.
`]`       |	If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching `[` command.

(Alternatively, the `]` command may instead be translated as an unconditional jump **to** the corresponding `[` command, or vice versa; programs will behave the same but will run more slowly, due to unnecessary double searching.)

`[` and `]` match as parentheses usually do: each [ matches exactly one `]` and vice versa, the `[` comes first, and there can be no unmatched `[` or `]` between the two.

Brainfuck programs can be translated into [C](https://en.wikipedia.org/wiki/C_(programming_language)) using the following substitutions, assuming `ptr` is of type `char*` and has been initialized to point to an array of zeroed bytes:

brainfuck command	| C equivalent
---               | ---
(Program Start)	  | `char array[30000] = {0}; char *ptr = array;`
`>`               |	`++ptr;`
`<`	              | `--ptr;`
`+`               | `++*ptr;`
`-`               |	`--*ptr;`
`.`               |	`putchar(*ptr);`
`,`               |	`*ptr = getchar();`
`[`               |	`while (*ptr) {`
`]`               |	`}`

## Examples
### Adding two values
As a first, simple example, the following code snippet will add the current cell's value to the next cell: Each time the loop is executed, the current cell is decremented, the data pointer moves to the right, that next cell is incremented, and the data pointer moves left again. This sequence is repeated until the starting cell is 0.

```brainfuck
[->+<]
```
This can be incorporated into a simple addition program as follows:
```brainfuck
++       Cell c0 = 2
> +++++  Cell c1 = 5

[        Start your loops with your cell pointer on the loop counter (c1 in our case)
< +      Add 1 to c0
> -      Subtract 1 from c1
]        End your loops with the cell pointer on the loop counter

At this point our program has added 5 to 2 leaving 7 in c0 and 0 in c1
but we cannot output this value to the terminal since it is not ASCII encoded.

To display the ASCII character "7" we must add 48 to the value 7.
We use a loop to compute 48 = 6 * 8.

++++ ++++  c1 = 8 and this will be our loop counter again
[
< +++ +++  Add 6 to c0
> -        Subtract 1 from c1
]
< .        Print out c0 which has the value 55 which translates to "7"!
```
### Hello World!
The following program prints "Hello World!" and a newline to the screen:

```brainfuck
[ This program prints "Hello World!" and a newline to the screen, its
  length is 106 active command characters. [It is not the shortest.]

  This loop is an "initial comment loop", a simple way of adding a comment
  to a BF program such that you don't have to worry about any command
  characters. Any ".", ",", "+", "-", "<" and ">" characters are simply
  ignored, the "[" and "]" characters just have to be balanced. This
  loop and the commands it contains are ignored because the current cell
  defaults to a value of 0; the 0 value causes this loop to be skipped.
]
++++++++               Set Cell #0 to 8
[
    >++++               Add 4 to Cell #1; this will always set Cell #1 to 4
    [                   as the cell will be cleared by the loop
        >++             Add 2 to Cell #2
        >+++            Add 3 to Cell #3
        >+++            Add 3 to Cell #4
        >+              Add 1 to Cell #5
        <<<<-           Decrement the loop counter in Cell #1
    ]                   Loop until Cell #1 is zero; number of iterations is 4
    >+                  Add 1 to Cell #2
    >+                  Add 1 to Cell #3
    >-                  Subtract 1 from Cell #4
    >>+                 Add 1 to Cell #6
    [<]                 Move back to the first zero cell you find; this will
                        be Cell #1 which was cleared by the previous loop
    <-                  Decrement the loop Counter in Cell #0
]                       Loop until Cell #0 is zero; number of iterations is 8

The result of this is:
Cell no :   0   1   2   3   4   5   6
Contents:   0   0  72 104  88  32   8
Pointer :   ^

>>.                     Cell #2 has value 72 which is 'H'
>---.                   Subtract 3 from Cell #3 to get 101 which is 'e'
+++++++..+++.           Likewise for 'llo' from Cell #3
>>.                     Cell #5 is 32 for the space
<-.                     Subtract 1 from Cell #4 for 87 to give a 'W'
<.                      Cell #3 was set to 'o' from the end of 'Hello'
+++.------.--------.    Cell #3 for 'rl' and 'd'
>>+.                    Add 1 to Cell #5 gives us an exclamation point
>++.                    And finally a newline from Cell #6
```
For "readability", this code has been spread across many lines, and blanks and comments have been added. Brainfuck ignores all characters except the eight commands +-<>[],. so no special syntax for comments is needed (as long as the comments do not contain the command characters). The code could just as well have been written as:
```brainfuck
++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
```
## Credits
 
> [Wikipedia](https://en.wikipedia.org/wiki/Brainfuck), this website help me to write this little documentation.

> [This post](https://medium.com/@thelukaswils/making-a-brainf-ck-to-c-compiler-in-rust-10f0c01a282d), this post help me to write the script's part of transpiler.

> [This post](https://benkonz.github.io/building-a-brainfuck-compiler-with-rust-and-llvm/), this post gelp me to write the script's part of CLI.

Copyright © 2022 - Zuygui - Under MIT licence
