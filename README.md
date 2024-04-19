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
```

"AddExe" and "Source" should be at the start in the order.
"LinkDir" and "IncludeDir" should be before you write "Link".
The command for that would be:

```sh
gcc -o prender prender.c -Llib -Iinclude -lSDL -lSDLmain -lc -lm
```
