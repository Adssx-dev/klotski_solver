# klotski_solver
Simple solver for the "Klotski" game written in Rust

# Game presentation


![Picture of Klotski board](https://github.com/Adssx-dev/klotski_solver/blob/master/klotski_picture.png)

## Rules

Klotski is a game where pieces of wood are places in a 5 row by 4 columns board.
 - Pieces can move on the board using the empty space. 
 - They cannot overlap and cannot get out of the board.
 - They can only move horizontally and vertically
 - They must move to integer coordinates

The goal of the game is to get the largest piece (the 2x2 square) to the bottom of the board.

# Technical solution

## Algorithm

The algorithm used here is a simple search in a space of solutions using a tree of parent/children board states

## Output

The ooutput is a serie of board displayed in the console, pressing Enter displays the next state to get to the solution.
The board are represented using ASCII characters, each piece having a special character : 
- S represents a small square (1x1)
- V a vertical piece (2x1)
- H the horizontal piece (1x2)
- G the Great square (2x2)

For instance, the initial board is represented by this ASCII board : 
VGGV  
VGGV  
.HH.  
VSSV  
VSSV  
