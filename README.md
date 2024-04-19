# Mininal build system for C
## Here an example:

```CMake
AddExe prender
Source prender.c
LinkDir lib
IncludeDir include
Link SDL
Link SDLmain
Link c
Link m

==~==

AddExe otherstuff
Source otherstuff.c
LinkDir .
Link prender
```

"AddExe" and "Source" should be at the start in the order.
"LinkDir" and "IncludeDir" should be before you write "Link".
The command sfor that would be:

```bash
gcc -o prender prender.c -Llib -Iinclude -lSDL -lSDLmain -lc -lm
```
```bash
gcc -o otherstuff otherstuff.c -L -lprender
```

"==~==" is used to split sections.
