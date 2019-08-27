# GTA San Andreas WASD cheat finder

Cheats in GTA:SA work quite funnily: each time you press a key, based on the last 6-29 characters you pressed, a CRC32
hash is calculated, and compared against a table. Since 32 bits are not that much, it is really easy to find some 
collisions. The most notable example is "HESOYAM", which backwards has the following CRC: `0xEECCEA2B`, same as
 "INEEDSOMEHELP", which was actually the intended cheat code.

Another side effect of this, besides finding shorter versions of codes, is that even just by pressing the WASD keys, you
might accidentally activate a cheat, for example, "WWDWASWWDWAS" is the same as "DULLDULLDAY" (overcast weather)

This project is supposed to find all the WASD cheat codes.
It runs paralelly on 4 threads.