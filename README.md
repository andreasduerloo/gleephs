# gleephs

**WORK IN PROGRESS**

**gleephs** is inspired by the (hiero)glyphs used in Nintendo's _The Legend of Zelda: Breath of the Wild_.

Looking at images of those glyphs, a couple of rules can be deduced:
* Every glyph is a grid of 4 x 4 'dots'.
* These dots _can_ be connected to their vertical or horizontal neighbors.
* A dot cannot be connected to more than two of its neighbors.

Although theoretically a glyph could consist of 16 unconnected dots or 8 (minimum length) "snakes", the art in the game shows a number of clear preferences:
* Most glyphs have one, two, three or four "snakes" of varying length.
* At most there are four unconnected dots, the rest of the glyph is taken up by snakes.

## What does it do?

**gleephs** takes an MD5 hash as its input. This hash is then converted into a glyph. Care has been taken to retain some hash-like qualities:
* The same input will yield the same output
* Different input will yield a different output
* The input hash can't be deduced from the output glyph

## Examples

5eb63bbbe01eeed093cb22bb8f5acdc3

▓▓  ▓▓  ▓▓▓▓▓▓  
    ▓▓          
▓▓▓▓▓▓  ▓▓  ▓▓  
        ▓▓      
▓▓  ▓▓  ▓▓▓▓▓▓  
  
▓▓▓▓▓▓  ▓▓▓▓▓▓  


760df4a2857daeff08c875beb2dcb091

▓▓▓▓▓▓  ▓▓  ▓▓  
        ▓▓      
▓▓  ▓▓  ▓▓▓▓▓▓  
  
▓▓  ▓▓▓▓▓▓▓▓▓▓  
▓▓  ▓▓    
▓▓  ▓▓  ▓▓▓▓▓▓  


59535ae31d32e29034b6944c402f3a0c

▓▓▓▓▓▓▓▓▓▓  ▓▓  
▓▓          ▓▓  
▓▓  ▓▓▓▓▓▓  ▓▓  
▓▓      ▓▓      
▓▓  ▓▓  ▓▓  ▓▓  
  
▓▓  ▓▓▓▓▓▓▓▓▓▓  


5b7249818459595a81f84722db2bdb49

▓▓  ▓▓  ▓▓  ▓▓  
            ▓▓  
▓▓  ▓▓  ▓▓  ▓▓  
▓▓      ▓▓  ▓▓  
▓▓  ▓▓  ▓▓  ▓▓  
▓▓  ▓▓      ▓▓  
▓▓  ▓▓  ▓▓▓▓▓▓  