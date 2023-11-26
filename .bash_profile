#
# ~/.bash_profile
#

if [[ "$(tty)"="/dev/tty1" ]]; then
    read -n1 -p "Enable windows mode? [Y,n]" doit 
    case $doit in  
        n|N) echo '' ;;
    	*) startx "$HOME/.xinitrc" ;;
    esac 
fi

[[ -f ~/.bashrc ]] && . ~/.bashrc
