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
