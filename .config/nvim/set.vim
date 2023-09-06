set number
set syntax
set title  " Muestra el nombre del archivo en la ventana de la terminal
" set number  " Muestra los números de las líneas
set mouse=a  " Permite la integración del mouse (seleccionar texto, mover el cursor)

set nowrap  " No dividir la línea si es muy larga

set cursorline  " Resalta la línea actual

" Indentación a 4 espacios
set shiftwidth=4

set hidden  " Permitir cambiar de buffers sin tener que guardarlos
" set ignorecase  " Ignorar mayúsculas al hacer una búsqueda
" set smartcase  " No ignorar mayúsculas si la palabra a buscar contiene mayúsculas

set spelllang=en,es  " Corregir palabras usando diccionarios en inglés y español

" set termguicolors  " Activa true colors en la terminal
" set background=dark  " Fondo del tema: light o dark

resize -N 50
let g:rnvimr_enable_picker = 1
let g:rnvimr_shadow_winblend = 100
let NERDTreeShowHidden=1
let NERDTreeIgnore=['\.gitignore$', '\.git$'] " ignore files in nerd tree
let g:NERDTreeChDirMode = 2
