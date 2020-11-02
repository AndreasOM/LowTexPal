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
