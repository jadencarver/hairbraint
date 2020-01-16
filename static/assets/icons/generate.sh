#!/bin/bash

ls -1 originals/*.svg | while read ORIG; do
  BASENAME=`basename $ORIG`
  NAME="${BASENAME%.svg}"
  OUTPUT="$NAME.png"
  echo "$NAME: Generating $OUTPUT from $ORIG"
  convert \
    -background none \
    $ORIG \
    -alpha deactivate \
    -negate \
    -trim \
    -resize 128x128 \
    -gravity center \
    -extent 128x128 \
    $OUTPUT

done
