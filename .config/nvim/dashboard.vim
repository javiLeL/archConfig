lua<<END
-- vim.opt.rtp:append('~/workspace/dashboard-nvim')
-- vim.opt.rtp:append('~/workspace/dashboard-doom')
require('dashboard').setup{
    theme = 'doom',
    config = {
	header = {
	    '',
 	    ' ',
 	    ' ',
 	    ' ▄▄▄██▀▀▀▄▄▄    ██▒   █▓ ██▓      ██▓    ▓█████  ██▓     ',
 	    '   ▒██  ▒████▄ ▓██░   █▒▓██▒     ▓██▒    ▓█   ▀ ▓██▒     ',
 	    '   ░██  ▒██  ▀█▄▓██  █▒░▒██▒     ▒██░    ▒███   ▒██░     ',
 	    '▓██▄██▓ ░██▄▄▄▄██▒██ █░░░██░     ▒██░    ▒▓█  ▄ ▒██░     ',
 	    ' ▓███▒   ▓█   ▓██▒▒▀█░  ░██░ ██▓ ░██████▒░▒████▒░██████▒ ',
 	    ' ▒▓▒▒░   ▒▒   ▓▒█░░ ▐░  ░▓   ▒▓▒ ░ ▒░▓  ░░░ ▒░ ░░ ▒░▓  ░ ',
 	    ' ▒ ░▒░    ▒   ▒▒ ░░ ░░   ▒ ░ ░▒  ░ ░ ▒  ░ ░ ░  ░░ ░ ▒  ░ ',
 	    ' ░ ░ ░    ░   ▒     ░░   ▒ ░ ░     ░ ░      ░     ░ ░    ',
 	    ' ░   ░        ░  ░   ░   ░    ░      ░  ░   ░  ░    ░  ░ ',
 	    '                    ░         ░                          ',
 	    ' ',
 	    ' ',
	    '',
	},
	center = {
      {	
        icon = ' ',
        icon_hl = 'Title',
        desc = 'Open File                  ',
        -- desc_hl = 'String',
        key = 'o',
        keymap = 'SPC f f',
        key_hl = 'Number',
        action = ':RnvimrToggle'
      },
      {
        icon = ' ',
        desc = 'Change Setting',
        key = 'c',
        keymap = 'SPC f d',
        action = ':e ~/.config/nvim'
      },
    },
    footer = {},
    },
}

-- vim.api.nvim_set_hl(0, 'DashboardHeader', {
--  fg = 'green',
-- })
END
