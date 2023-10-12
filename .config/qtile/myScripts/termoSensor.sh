#!/bin/bash

DATAFILE=~/Document/.data/dataTermica.txt
FILELINES=120
# get temp1 information critical temerature and id in this case 1 (temp1 = 1:, temp2 = 2...)
INFORMATION=($(sensors | grep temp1 | grep -Po '[0-9]+..'))

if [ "$1" = 'start' ]; then
    echo 0 > $DATAFILE
    for ((i=0;i<=$FILELINES;i++)); do
	echo 0 >> $DATAFILE
    done
elif [ "$1" = 'add' ]; then
    str=${INFORMATION[1]}
    tem=${str%??}
    echo $tem
    echo $tem >> $DATAFILE
    sed -i '1d' $DATAFILE
else
    WIDTH=$(($(/usr/bin/tput cols)-10))
    HEIGHT=$(($(/usr/bin/tput lines)-10))
    str=${INFORMATION[2]}
    CRITICAl=${str%??}
    TEMS=()
    COUNT=0

    declare -A screen
    
    for ((i=1;i<=$FILELINES;i++)); do
	TEMS+=($(sed "$i!d" $DATAFILE))
    done
    
    for ((i=0;i<=HEIGHT;i++)) do
	for ((j=0;j<=WIDTH;j++)) do
	    temeratureGraphic=$((($HEIGHT*${TEMS[$((((${#TEMS[@]}*$j)-1)/$WIDTH))]}/$CRITICAl)))
	    if [ $temeratureGraphic -ge $i ] && [ $temeratureGraphic -ne 0 ]; then
		screen[$i,$j]="#" 
	    else
		screen[$i,$j]="."
	    fi
	done
    done

    for ((i=0;i<=HEIGHT;i++)) do
	for ((j=0;j<=WIDTH;j++)) do
	    echo -e ${screen[$(($HEIGHT-$i-1)),$j]}'\c' 
	done
	if [ $((($CRITICAl*($HEIGHT-$i-1))/($HEIGHT))) -ge 0 ]; then
	    if [ $((($CRITICAl*($HEIGHT-$i-1))/($HEIGHT))) -le 9 ]; then
		echo ' 0'$((($CRITICAl*($HEIGHT-$i-1))/($HEIGHT)))'ºC'
	   else
	        echo ' '$((($CRITICAl*($HEIGHT-$i))/($HEIGHT)))'ºC' 
	    fi
	fi
    done
    # echo ${TEMS[*]} 
    # echo $((($WIDTH*5)/${#TEMS[*]})) # Operacion anchura tiempo
    # echo $((($HEIGHT*50)/$CRITICAl)) # Operacion altura temeratura
    # echo ${screen[30,50]}
fi
