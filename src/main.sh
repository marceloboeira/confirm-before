#!/usr/bin/env bash

cr=`echo $'\n.' > /dev/tty`
cr=${cr%.}

read -p "Do you want to run $*? [N/y]${cr}${cr}" -s < /dev/tty

if test "$REPLY" = "y" -o "$REPLY" = "Y"; then
    exec "$@"
else
    >&2 echo "Cancelled by user" > /dev/tty
fi
