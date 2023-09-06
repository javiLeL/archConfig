let g:mapleader = ' '  " Definir espacio como la tecla líder

" Ctrl+y to copy in clip board
map <C-Y> "+y<CR>
map <C-C> "+y<CR>

" Ctrl+x to cut
map <C-X> "+d<CR>

" Ctrl+v to paste in clip board
map <C-V> "+p<CR>

" Ctrl+z to undo
map <C-Z> :undo<CR>

" Ctrl+z to redo
map <C-X> :redo<CR>

" Next window
map <C-N> :bnext<CR>

" Before window
map <C-B> :bprev<CR>

" Close window
map <C-Q> :bdelete<CR>

" Toggle on/off nerd tree
map <C-T> :NERDTreeToggle .<CR>

" Ranger
nnoremap <M-t> :RnvimrToggle<CR>

" Dashboard
map <C-H> :Dashboard<CR>
