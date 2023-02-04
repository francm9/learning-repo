
-------CONFIGURACION EN LUA DE VIM--------

--Importar el archivo con la configuracion basica--
require('basic_config')

--Importa el archivo con la configuracion de los plugins--
require('plug_install')

--Importa el archivo con la configuracion de color-- 
require('color_theme')

--Importa las configuraciones de los plugins--
require('plugins/treesiter')
require('plugins/lualine')
require('plugins/nerdtree')
require('plugins/telescope')

--Importa los atajos de teclado--
require('keymapping')

