#
# ~/.bashrc
#

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

# alias ls='ls --color=auto'
alias cat='bat'
alias ls='lsd'
alias grep='grep --color=auto'
# PS1='󰣇 \u@\h \W\$ '
PS1='\[\e[38;5;27m\]󰣇 \[\e[32;1m\]\u\[\e[0;38;5;46m\]@\[\e[32;1m\]\h \[\e[0m\]\w\[\e[97m\]\\$ \[\e[0m\]'
