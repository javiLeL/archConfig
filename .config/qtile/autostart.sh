#!/bin/sh

# fondo panatlla
nitrogen --restore &

# transparencias
picom &

# Activar el touch del touchpad
synclient TapButton1=1 &
~/.config/qtile/myScripts/trackpad-toggle rest &

# desactiva bluetooth
bluetoothctl power off &
# xrandr --output eDP1 --mode 1920x1080 --rate 60 

# auto felititation
# ~/Document/happybirday.sh
