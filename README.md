# LowTexPal


Minimalistic tool to manipulate images that are used for low poly texturing.

## Examples

```
lowtexpal -f example_01.png add-color --color "#000000"
lowtexpal -f example_01.png add-color --color "#ffffff"
lowtexpal -f example_01.png add-color --color "#ff0000"
lowtexpal -f example_01.png add-color --color "#00ff00"
lowtexpal -f example_01.png add-color --color "#0000ff"
lowtexpal -f example_01.png add-color --color "#ffff00"
lowtexpal -f example_01.png add-color --color "#ff00ff"
lowtexpal -f example_01.png add-color --color "#00ffff"
```

![Example Image 01](example_01.png)

Scaled up:

![Example Image 01 - Big ](big_example_01.png)

```
#!/bin/sh

png="example_02.png"
rm ${png}

lowtexpal -f ${png} add-color --color "black"
lowtexpal -f ${png} add-color --color "white"
lowtexpal -f ${png} add-color --color "red"
lowtexpal -f ${png} add-color --color "lime"
lowtexpal -f ${png} add-color --color "blue"
lowtexpal -f ${png} add-color --color "yellow"
lowtexpal -f ${png} add-color --color "fuchsia"
lowtexpal -f ${png} add-color --color "cyan"

gm convert ${png} -filter point -resize 128x128 big_${png}
```

![Example Image 02](example_02.png)

Scaled up:

![Example Image 02 - Big ](big_example_02.png)

```
#!/bin/sh

png="example_03.png"
rm ${png}

lowtexpal -f ${png} add-gradient --start-color "black" --end-color "white" --steps "4"
lowtexpal -f ${png} add-gradient --start-color "yellowgreen" --end-color "thistle" --steps "12"

lowtexpal -f ${png} add-gradient --start-color "black" --end-color "blue" --steps "4"
lowtexpal -f ${png} add-gradient --start-color "blue" --end-color "skyblue" --steps "8"
lowtexpal -f ${png} add-gradient --start-color "skyblue" --end-color "white" --steps "4"

lowtexpal -f ${png} add-gradient --start-color "black" --end-color "red" --steps "112"
lowtexpal -f ${png} add-gradient --start-color "red" --end-color "white" --steps "112"

gm convert ${png} -filter point -resize 128x128 big_${png}
```

![Example Image 03](example_03.png)

Scaled up:

![Example Image 03 - Big ](big_example_03.png)

