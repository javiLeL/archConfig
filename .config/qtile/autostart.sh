#!/bin/sh

# fondo panatlla
# nitrogen --restore &
nitrogen --set-zoom-fill --random ~/Pictures/backgrounds/ --head=0 > /dev/null

# transparencias
picom &

# Activar el touch del touchpad
synclient TapButton1=1 &
~/.config/qtile/myScripts/trackpad-toggle rest &

# desactiva bluetooth
bluetoothctl power off &

# Set 60Hz
sleep 1
xrandr --output eDP-1 --mode 1920x1080 --rate 60 & 
