# gleephs

**gleephs** is inspired by the glyphs used in Nintendo's _The Legend of Zelda: Breath of the Wild_.

Looking at images of those glyphs, a couple of rules can be deduced:
* Every glyph is a grid of 4 x 4 'dots'.
* These dots _can_ be connected to their vertical or horizontal neighbors. There are no diagonal connections.
* A dot can be connection to two, one, or none of its neighbors.

Although theoretically a glyph could consist of 16 unconnected dots or 8 (minimum length) "snakes", the art in the game shows a number of clear preferences:
* Most glyphs have one, two, three or four "snakes" of varying length.
* At most there are four unconnected dots, the rest of the glyph is taken up by snakes.
These two 'rules' are not enforced by the code. 

## What does it do?

**gleephs** takes an MD5 hash as its input. This hash is then converted into a glyph. Care has been taken to retain some hash-like qualities:
* The same input will yield the same output
* The input hash can't be deduced from the output glyph
Of course, because there are far fewer possible glyphs than MD5 hashes, collisions (i.e. different inputs with identical outputs) are far more likely.

The algorithm works as follows:
- The input hash is 'folded' into a 64-bit number by XOR'ing its two halves.
- That 64-bit number is then 'squashed' into a 16-bit number by taking the least significant bit of each four-bit group.
- Now we check for each bit how many of its neighbors have the same value (0 or 1). If more than two neighbors have the same value, the bit is flagged to be flipped since it violates the rules.
- If any bits are set to be flipped, they're flipped and we check for neighbors again.
- If no bits need to be flipped, neighbors with the same values are connected and the glyph is drawn.

Note that this might mean the algorithm can get into a loop.

## Examples

By blind luck, I have ran into a few pleasingly symmetric gleephs.
My first and last name, hashed and then gleephed:
```
▓▓▓▓▓▓▓▓▓▓  ▓▓
        ▓▓      
▓▓▓▓▓▓  ▓▓▓▓▓▓
▓▓          ▓▓
▓▓  ▓▓  ▓▓  ▓▓
▓▓      ▓▓  ▓▓
▓▓▓▓▓▓▓▓▓▓  ▓▓
```

My fiancée's first name and mine:
```
▓▓  ▓▓  ▓▓▓▓▓▓
▓▓          ▓▓
▓▓▓▓▓▓  ▓▓  ▓▓
    ▓▓
▓▓  ▓▓▓▓▓▓  ▓▓
▓▓      ▓▓
▓▓▓▓▓▓  ▓▓▓▓▓▓
```

In general, the output is less oganized. See for example my fiancée's first and last name:
```
▓▓  ▓▓▓▓▓▓▓▓▓▓
            ▓▓  
▓▓  ▓▓▓▓▓▓  ▓▓
        ▓▓
▓▓  ▓▓  ▓▓▓▓▓▓
▓▓  ▓▓
▓▓  ▓▓▓▓▓▓▓▓▓▓
```

And - unfortunately - 'gleephs' itself returns an unattractive glyph with lots of free nodes:
```
▓▓  ▓▓▓▓▓▓  ▓▓

▓▓  ▓▓▓▓▓▓  ▓▓
    ▓▓
▓▓▓▓▓▓  ▓▓  ▓▓
▓▓
▓▓  ▓▓  ▓▓  ▓▓
```
