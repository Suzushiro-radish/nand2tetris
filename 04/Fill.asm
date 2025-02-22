// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.

(LOOP)
// Read key
@KBD
D=M

// if key in pressed (D=0), set color = -1
@ELSE
D;JEQ
@color
M=-1
@SCR_START
0;JMP

// else set color = 0
(ELSE)
@color
M=0

(SCR_START)

// i = 0
@i
M=0

(SCR_LOOP)
// Locate screen
@SCREEN
D=A
@i
D=D+M
@loc
M=D

// set color
@color
D=M
@loc
A=M
M=D

// if (i >= 8192) goto FIN_SCR
@i
D=M
@8191
D=D-A
@FIN_SCR
D;JGE

// i ++
@i
M=M+1

@SCR_LOOP
0;JMP
// SCR_LOOP end

(FIN_SCR)
@LOOP
0;JMP


