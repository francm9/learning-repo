--CONFIGURACION BASICA DE NEOVIM--

--Definir el set de vim con vim.opt--
local set = vim.opt

--Configuracion Basica--
set.number = true			    --Muestra el numero de fila
set.relativenumber = true
set.title = true		    	--Muestra el nombre del archivo
set.mouse = "a"			    	--Dos opciones: a y r. Con a puedo moverme por donde quiera
set.cursorline = true			--Resalta la linea en la que estoy
set.ignorecase = true			--Ignora las mayusculas al hacer busquedas
set.spelllang = "es","en"		--Corrige usando diccionarios de ingles y español
set.background = "dark"			--Fondo de neovim -> Dark o Light
set.tabstop = 4
set.shiftwidth = 4
set.expandtab = true 			--Identacion
set.wrap = false                --No divide la linea al final de la ventana del editor
set.encoding = "UTF-8"          --Codifica en UTF-8

