--Crear la funcion de mapeo--
function map(mode, lhs, rhs, opts)
	local options = { noremap = true }
	if opts then
		options = vim.tbl_extend("force", options, opts)
	end
	vim.api.nvim_set_keymap(mode, lhs, rhs, options)
end

vim.g.mapleader = ","

--NerdTree--
map("n", "<F2>", ":NERDTreeToggle<CR>")
-- Nerd commenter-
vim.g.NERDCreateDefaultMappings = 1

--Ejecucion de archivos--
map("n", "<F3>", ":!gcc %<CR> :!./a.out<CR>")         --Compila en C y ejecuta]]
map("n", "<F4>", ":!rm a.out<CR>")                    --Elimina el a.out]]
map("n", "<F5>", ":!python %<CR>")                    --Ejecuta .py]]
map("n","<C-f>", ":Telescope find_files <CR>")        --Busqueda de Telescope en el directorio abierto]]
map("n", "<C-b>", ":Telescope buffers <CR>")

--Guardar archivos y cerrar neovim
map("n","<C-s>",":w <CR>")                            --Guarda el archivo con C-s
map("n","<C-q>",":q <CR>")                            --Cierra neovim
map("n","<C-d>",":u <CR>")                              --Deshacer cambios  
