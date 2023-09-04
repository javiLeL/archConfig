call plug#begin()
" Themes
" Plug 'folke/tokyonight.nvim', { 'branch': 'main' }
Plug 'bluz71/vim-nightfly-colors', { 'as': 'nightfly' }
" Plug 'patstockwell/vim-monokai-tasty'

" Coc auto help
Plug 'neoclide/coc.nvim', {'branch': 'release'}

" Tree
Plug 'preservim/nerdtree'

"Lua line
Plug 'nvim-lualine/lualine.nvim'

" If you want to have icons in your statusline choose one of these
" Plug 'kyazdani42/nvim-web-devicons'
Plug 'camspiers/animate.vim'
Plug 'camspiers/lens.vim'

" Superior tables
Plug 'ap/vim-buftabline'

" Icons 
Plug 'ryanoasis/vim-devicons'

" Ranger
Plug 'kevinhwang91/rnvimr'
" Plug 'francoiscabrol/ranger.vim'
" Plug 'rbgrouleff/bclose.vim'

Plug 'glepnir/dashboard-nvim'
call plug#end() 

" colorscheme tokyonight-moon
colorscheme nightfly
" colorscheme vim-monokai-tasty
