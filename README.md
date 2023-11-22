# Byte Whisperer

## Instruction sets

### LDA
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### STR
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### PUSH
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### POP
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### AND
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### OR
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### NOT
- 5 bits for instruction
- 2 bits for the address of the register

### ADD
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### SUB
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### DIV
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### MUL
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### MOD
- 5 bits for instruction
- 2 bits for the address of the register
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register

### INC
- 5 bits for instruction
- 2 bits for the address of the register

### DEC
- 5 bits for instruction
- 2 bits for the address of the register

### BEQ
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register
- 12 bits for second parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register
- 3 bits for the address of the jump

### BNE
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register
- 12 bits for second parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register
- 3 bits for the address of the jump

### BBG
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register
- 12 bits for second parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register
- 3 bits for the address of the jump

### BSM
- 5 bits for instruction
- 12 bits for parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register
- 12 bits for second parameter
	- 2 bits for type of the parameter
	- 10 bits for the address in the memory, the constant or the address of the second register
- 3 bits for the address of the jump

### JMP
- 5 bits for instruction
- 3 bits for the address of the jump

### HLT
- 5 bits for instruction

## Others

### Label
- 5 bits for instruction
- 3 bits for label name

### Category
- 30 bits for instruction
- 2 bits for category name

## Variable definition

### Variable
- 22 bits for variable name
- 10 bits for constant data
