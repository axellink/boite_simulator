# Shut the box
For those who may not know, this is a "simulator" for the game 
[shut the box](https://en.wikipedia.org/wiki/Shut_the_box) in order to try to find the best strategy
not to lose the game.

The variant I'm used to play however, forces you to sum your dices dots, and to select
one or two values that also adds up to the previous sum.

This is different from what wikipedia says where you can sum or subtract the dots, and then
choose whatever available digits that adds up to your previous calculus.

# Purpose
There are actually two purposes for this little program.

First is to really take over my opponents in this game constantly !

The second, and the principle one, is to begin writing rust on a small project which took me
an afternoon. I may have to add unit tests, maybe, if I take time at some point !!!

# Results
Since some of you may be interested in the results I have come to, here they are :
```ignorelang
Testing random strategy
Average score : 5.957
Average number of perfect : 0.111

Testing highest first strategy
Average score : 7.911
Average number of perfect : 0.268

Testing double lowest strategy
Average score : 5.249
Average number of perfect : 0.078

Testing double highest strategy
Average score : 5.347
Average number of perfect : 0.081

Testing even highest strategy
Average score : 5.962
Average number of perfect : 0.138

Testing odd highest strategy
Average score : 6.279
Average number of perfect : 0.162
```
See the strategies code if you want to know more.