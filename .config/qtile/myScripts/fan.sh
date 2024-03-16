#!/bin/bash
# Set de profile of the Laptop to the next estatus
asusctl profile -n

# We get the Status of the fan and send a notification with the information
FANMODE=$(asusctl profile -p | cut -d ' ' -f4)

# If the fan mode is some of this load a differnts logo for it
case $FANMODE in
    Quiet)
        ICON='󰠝'
        ;;
    Balanced)
        ICON='󱑰'
        ;;
    Performance)
        ICON='󰈐'
        ;;
esac
notify-send -a some_fan_notifiers "$ICON  Fan Mode" "$FANMODE" -r 2221