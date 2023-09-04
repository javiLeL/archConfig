#
# ~/.bash_profile
#

if [[ "$(tty)"="/dev/tty1" ]]; then
	startx "$HOME/.xinitrc"
fi

[[ -f ~/.bashrc ]] && . ~/.bashrc
