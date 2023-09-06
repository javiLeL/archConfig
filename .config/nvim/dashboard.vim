lua<<END
-- vim.opt.rtp:append('~/workspace/dashboard-nvim')
-- vim.opt.rtp:append('~/workspace/dashboard-doom')
require('dashboard').setup{
    theme = 'doom',
    config = {
	header = {
	    '',
	    '',
 	    ' ',
 	    ' ',
 	    ' ▄▄▄██▀▀▀▄▄▄    ██▒   █▓ ██▓      ██▓    ▓█████  ██▓    ',
 	    '   ▒██  ▒████▄ ▓██░   █▒▓██▒     ▓██▒    ▓█   ▀ ▓██▒    ',
 	    '   ░██  ▒██  ▀█▄▓██  █▒░▒██▒     ▒██░    ▒███   ▒██░    ',
 	    '▓██▄██▓ ░██▄▄▄▄██▒██ █░░░██░     ▒██░    ▒▓█  ▄ ▒██░    ',
 	    ' ▓███▒   ▓█   ▓██▒▒▀█░  ░██░ ██▓ ░██████▒░▒████▒░██████▒',
 	    ' ▒▓▒▒░   ▒▒   ▓▒█░░ ▐░  ░▓   ▒▓▒ ░ ▒░▓  ░░░ ▒░ ░░ ▒░▓  ░',
 	    ' ▒ ░▒░    ▒   ▒▒ ░░ ░░   ▒ ░ ░▒  ░ ░ ▒  ░ ░ ░  ░░ ░ ▒  ░',
 	    ' ░ ░ ░    ░   ▒     ░░   ▒ ░ ░     ░ ░      ░     ░ ░   ',
 	    ' ░   ░        ░  ░   ░   ░    ░      ░  ░   ░  ░    ░  ░',
 	    '                    ░         ░                         ',
	    '',
 	    '',
 	    '',
	},
	center = {
	    {	
              icon = ' ',
              -- icon_hl = 'Title',
              desc = 'New File                  ',
              -- desc_hl = 'String',
              key = 'n',
              keymap = 'SPC f f',
              -- key_hl = 'Number',
              action = 'e ~/?'
	    },
	    {	
              icon = ' ',
              desc = 'Open File',
              key = 'o',
              keymap = 'SPC f f',
              action = ':RnvimrToggle'
            },
	    {	
              icon = ' ',
              desc = 'Open Folder',
              key = 'k',
              keymap = 'SPC f f',
              action = ':NERDTreeToggle'
	    },
	    {	
              icon = ' ',
              desc = 'Find File',
              key = 'f',
              keymap = 'SPC f f',
              action = ':Telescope find_files'
	    },
	    {
              icon = ' ',
              desc = 'Change Setting',
              key = 'c',
              keymap = 'SPC f d',
              action = ':execute ":cd ~/.config/nvim | e ~/.config/nvim"'
            },
            {
              icon = ' ',
              desc = 'Last File',
              key = 'l',
              keymap = 'SPC f d',
              action = ':e #<1'
            },
          },
          footer = {},
    },
}

-- vim.api.nvim_set_hl(0, 'DashboardHeader', {
--  fg = 'green',
-- })
END
