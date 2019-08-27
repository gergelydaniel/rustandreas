# GTA San Andreas WASD cheat finder

Cheats in GTA:SA work quite funnily: each time you press a key, based on the last 6-29 characters you pressed, a CRC32
hash is calculated, and compared against a table. Since 32 bits are not that much, it is really easy to find some 
collisions. The most notable example is "HESOYAM", which backwards has the following CRC: `0xEECCEA2B`, same as
 "INEEDSOMEHELP", which was actually the intended cheat code.

Another side effect of this, besides finding shorter versions of codes, is that even just by pressing the WASD keys, you
might accidentally activate a cheat, for example, "WWDWASWWDWAS" is the same as "DULLDULLDAY" (overcast weather)

(Source of the cheats: https://gtagmodding.com/sanandreas/cheats/)

This project is supposed to find all the WASD cheat codes.
It runs paralelly on 4 threads.

Use `cargo run` to run the project.

Some results:

|Cheat code         |Effect                     |
|-------------------|---------------------------|
|WWDWASWWDWAS       |Overcast weather           |
|WSDWSWSWDDSWWA     |Cars float away when hit   |
|DWASAWSWSWASWD     |Rainy weather              |
|WASWSWASWWWWAA     |Spawn Romero               |
|ASDDAWADSASSAA     |Reduced traffic            |
|DDDSADWAWWWDAD     |All cars have nitro        |
|WWAASWAASSWDAS     |Clear wanted level         |
|AAAADADSAWADAW     |Spawn Quad                 |
|AWDAAWSWDSSSSA     |Spawn Vortex               |
|ADDAAWSSWSADSA     |Weapon Set 3               |
|SWDWDSSSSSDWAWD    |Spawn Romero               |
|WDWDWAWSSSADAWA    |Spawn Monster              |
|WSDWDWASWADWDWW    |Slower gameplay            |
|SWDSWDSAWSWDDWD    |Max driving skills         |
|DASDAWAAWSSWWAW    |Spawn Hunter               |
|SSWWDSSDASADWAW    |Health, Armor, $250k       |
|WAWSWWADDAWWAAA    |All green lights           |
|DWSWWASDASSWAAW    |Six wanted stars           |