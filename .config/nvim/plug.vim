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
Plug 'kyazdani42/nvim-web-devicons'
Plug 'camspiers/animate.vim'

" Superior tables
Plug 'ap/vim-buftabline'

" Icons 
Plug 'ryanoasis/vim-devicons'

" Ranger
Plug 'kevinhwang91/rnvimr'

" Dashboard
Plug 'glepnir/dashboard-nvim'

" telescope
Plug 'nvim-lua/plenary.nvim'
Plug 'nvim-telescope/telescope.nvim', { 'tag': '0.1.2' }

call plug#end() 
set hidden
" colorscheme tokyonight-moon
colorscheme nightfly
" colorscheme vim-monokai-tasty
