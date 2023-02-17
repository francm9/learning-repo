#!/bin/bash

WM_DESKTOP=$(xdotool getwindowfocus)

if [ $WM_DESKTOP == "483" ]; then

	echo "   Desktop "

elif [ $WM_DESKTOP != "1883" ]; then

	WM_CLASS=$(xprop -id $(xdotool getactivewindow) WM_CLASS | awk 'NF {print $NF}' | sed 's/"/ /g')
	WM_NAME=$(xprop -id $(xdotool getactivewindow) WM_NAME | cut -d '=' -f 2 | awk -F\" '{ print $2 }')

#    if [ $WM_NAME == "* NVIM" ]; then

#		echo "    Neovim "

	if [ $WM_CLASS == 'Alacritty' ]; then

		echo "   Alacritty "
	
	elif [ $WM_CLASS == 'firefox' ]; then

		echo "    Firefox "
    
    elif [ $WM_CLASS == 'Code' ]; then

		echo "    Visual Studio Code "
    elif [ $WM_CLASS == 'Xfce4-settings-manager' ]; then

		echo "    Ajustes "

    elif [ $WM_CLASS == 'discord' ]; then

		echo "    Discord "

    elif [ $WM_CLASS == 'Thunar' ]; then

		echo "    Thunar "

	else
		echo "   $WM_NAME "

	fi

fi
