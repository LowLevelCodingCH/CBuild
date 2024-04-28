# Mininal build system for C
## Here an example:

```CMake
ObjectOut
AddExe prender
Source prender.c
LinkDir lib
IncludeDir include
Link SDL
Link SDLmain
Link c
Link m

==~==
BuildStatic
AddExe otherstuff
Source otherstuff.c
LinkDir .
Link somelibrary
Link stdc++
```

"BuildStatic", "ObjectOut", "AddExe" and "Source" should be at the start in the order.
"LinkDir" and "IncludeDir" should be before you write "Link".
The commands for that would be:

```bash
gcc -c -o prender prender.c -Llib -Iinclude -lSDL -lSDLmain -lc -lm
```
```bash
gcc -o otherstuff otherstuff.c -L. -lotherstuff -lstdc++
```

"==~==" is used to split sections.
