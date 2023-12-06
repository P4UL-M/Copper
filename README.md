# Copper

## Table of Content
- [What is Copper ?](#what-is-copper)
- [How to use ?](#how-to-use)
- [Instruction](#instruction-sets)

## What is Copper ?

Copper is a assembly interpreter

## How to Use ?

To utilize Copper, follow the command line syntax and available options.

### Command Line Syntax:

```plaintext
copper <filename>
```

**Options:**
- **-h**, **--help**: Print this help message.
- **-V**, **--version**: Print version information.
- **-v**, **--verbose**: Enable verbose mode.
- **-d**, **--debug**: Enable debug mode.

**Commands:**

**Run Program:**
```plaintext
copper run <filename>
```
Execute the Copper program specified by <filename>.

**Export Program:**
```plaintext
copper export <filename> [<outputfile>]
```
Export the Copper program to a binary file. Optionally, you can specify an output file name. If no output file is provided, the default name is used.

**Examples:**

Run a Copper program:
```plaintext
copper program.co
```

Run a Copper program with verbose output:
```plaintext
copper -v run program.co
```

Export a Copper program to the default binary file:
```plaintext
copper export program.co
```

Export a Copper program to a specific binary file:
```plaintext
copper export program.co program.bin
```

## Instruction sets

Refer to the instruction sets below to find the specific functionalities and syntax of Copper.

### LDA \<reg1\> \<reg2\>/\<var\>/\<const\> - `0b00000` 
*Load register reg1 with the contents of either the contents of reg2, or the memory var or a constant const. Memory regions loads (load into a variable, for instance) are NOT ALLOWED.*
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### STR \<var\> \<reg\>/\<const\> - `0b00001` 
*Store in the memory position referred by var the value of register reg or a constant const. Register stores (store into register t0, for instance) are NOT ALLOWED.* 
- 5 bits for instruction
- 10 bits for the address of the variable
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for he constant or the address of the register
  	*No store from variable to variable allowed*

### PUSH \<reg\>/\<var\>/\<const\> - `0b00010` 
*Push to the top of the stack the contents of reg or var or a constant const.*
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### POP \<reg\> - `0b00011` 
*Pop from the top of the stack and store the value on reg. Storing in a memory region is NOT ALLOWED.*
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### AND \<reg1\> \<reg2\>/\<var\>/\<const\> - `0b00100` 
*Performs a logical AND operation between reg1 and a register reg2, a variable var or a constant const, and store the result on register reg1. Memory regions stores (store result into a variable, for instance) are NOT ALLOWED.*
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### OR \<reg1\> \<reg2\>/\<var\>/\<const\> - `0b00101` 
*Performs a logical OR operation between reg1 and a register reg2, a variable var or a constant const, and store the result on register reg1. Memory regions stores (store result into a variable, for instance) are NOT ALLOWED.*
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### NOT \<reg\> - `0b00110` 
*Performs a logical NOT operation on register reg and store the result on register reg. Memory regions stores (store result into a variable, for instance) are NOT ALLOWED.*
- 5 bits for instruction
- 2 bits for the address of the register

### ADD \<reg1\> \<reg2\>/\<var\>/\<const\> - `0b00111` 
*Performs the addition operation of reg1 and a register reg2, a variable var or a constant const, and store the result on register reg1. Memory regions stores (store result into a variable, for instance) are NOT ALLOWED.*
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### SUB \<reg1\> \<reg2\>/\<var\>/\<const\> - `0b01000` 
*Performs the subtraction operation of reg1 and a register reg2, a variable var or a constant const, and store the result on register reg1. The operation is given by second argument minus the first argument (i.e., reg2 – reg1). Memory regions stores (store result into a variable, for instance) are NOT ALLOWED.*
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### DIV \<reg1\> \<reg2\>/\<var\>/\<const\> - `0b01001` 
*Performs the integer division operation of reg1 and a register reg2, a variable var or a constant const, and store the result on register reg1. The operation is given by second argument divided by the first argument (i.e., reg2 / reg1). Memory regions stores (store result into a variable, for instance) are NOT ALLOWED.*
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### MUL \<reg1\> \<reg2\>/\<var\>/\<const\> - `0b01010` 
*Performs the integer multiplication operation of reg1 and a register reg2, a variable var or a constant const, and store the result on register reg1. Memory regions stores (store result into a variable, for instance) are NOT ALLOWED.*
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### MOD \<reg1\> \<reg2\>/\<var\>/\<const\> - `0b01011` 
*Performs the integer modulo operation of reg1 and a register reg2, a variable var or a constant cont, and store the result on register reg1. The operation is given by second argument modulo the first argument (i.e., reg2 mod  reg1). Memory regions stores (store result into a variable, for instance) are NOT ALLOWED.*
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### INC \<reg\> - `0b01100` 
*Increments the value of a register reg. Memory increments (incrementing a variable, for instance) are NOT ALLOWED.*
- 5 bits for instruction
- 2 bits for the address of the register

### DEC \<reg\> - `0b01101`
*Decrements the value of a register reg. Memory increments (decrementing a variable, for instance) are NOT ALLOWED.*
- 5 bits for instruction
- 2 bits for the address of the register

### BEQ \<reg1\>/\<var1\>/\<const1\> \<reg2\>/\<var2\>/\<const2\> \<LABEL\> - `0b01110` 
*Performs a comparison between two values, given by registers, variables or constants. Any combination is permitted. If they are equal, jump to the address defined by the label LABEL.*
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the register
- 12 bits for second parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register
- 3 bits for the address of the jump

### BNE \<reg1\>/\<var1\>/\<const1\> \<reg2\>/\<var2\>/\<const2\> \<LABEL\> - `0b01111` 
*Performs a comparison between two values, given by registers, variables or constants. Any combination is permitted. If they are different, jump to the address defined by the label LABEL.*
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the register
- 12 bits for second parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register
- 3 bits for the address of the jump

### BBG \<reg1\>/\<var1\>/\<const1\> \<reg2\>/\<var2\>/\<const2\> \<LABEL\> - `0b10000` 
Performs a comparison between two values, given by registers, variables or constants. Any combination is permitted. If the first parameter is bigger than the second parameter, jump to the address defined by the label LABEL.
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the register
- 12 bits for second parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register
- 3 bits for the address of the jump

### BSM \<reg1\>/\<var1\>/\<const1\> \<reg2\>/\<var2\>/\<const2\> \<LABEL\> -  `0b10001` 
*Performs a comparison between two values, given by registers, variables or constants. Any combination is permitted. If the first parameter is smaller than the second parameter, jump to the address defined by the label LABEL.*
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the register
- 12 bits for second parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register
- 3 bits for the address of the jump

### JMP \<LABEL\> - `0b10010` 
*Jump to the address defined by the label LABEL.*
- 5 bits for instruction
- 3 bits for the address of the jump

### SRL \<reg\> \<const\> - `0b10011` 
*This operation takes the value in reg and performs a logical shift left of the number of bits defined by the constant const. For instance, the value 0001 left shifted 1 time becomes.*
- 5 bits for instruction
- 2 bits for the address of the register
- 10 bits for the constant

### SRR \<reg\> \<const\> - `0b10100` 
*This operation takes the value in reg and performs a logical shift right of the number of bits defined by the constant const. For instance, the value 1000 right shifted 1 time becomes.*
- 5 bits for instruction
- 2 bits for the address of the register
- 10 bits for the constant

### HLT - `0b10101` 
*End the program execution.*
- 5 bits for instruction

### IN \<reg\>/\<var\> - `0b10110` 
*This operation take a value of the input stream and assign it to the parameter.*
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the register

### OUT \<reg\>/\<var\>/\<const\> - `0b10111` 
*This operation take the value from the parameter such as a constant, a variable or a register and write it in the output stream.*
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the register

### \<Label\>: - `0b11110`
*This operation mark the destination of a jump or a conditional jump.*
- 5 bits for instruction
- 3 bits for label name

## Others

### #Category - `0b11111` 
- 5 bits for instruction
- 2 bits for category name

### Variable definition
- 1 bits for data type
- 10 bits for variable name
- 10 bits for constant data

### Array definition
- 1 bits for data type
- 10 bits for array name
- 10 bits for array size
- 10 bits for constant data
