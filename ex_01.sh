#!/bin/sh

png="example_01.png"
rm ${png}

lowtexpal -f ${png} add-color --color "#000000"
lowtexpal -f ${png} add-color --color "#ffffff"
lowtexpal -f ${png} add-color --color "#ff0000"
lowtexpal -f ${png} add-color --color "#00ff00"
lowtexpal -f ${png} add-color --color "#0000ff"
lowtexpal -f ${png} add-color --color "#ffff00"
lowtexpal -f ${png} add-color --color "#ff00ff"
lowtexpal -f ${png} add-color --color "#00ffff"

gm convert ${png} -filter point -resize 128x128 big_${png}
