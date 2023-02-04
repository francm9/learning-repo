--Instalacion y configuracion de los plugins--
return require('packer').startup(function()

--Instalar packer--
use 'wbthomason/packer.nvim'

--Temas--
--Nord--
use 'shaunsingh/nord.nvim'
use 'jacoborus/tender.vim'
use 'Mofiqul/dracula.nvim'

--TreeSitter--
use {
	'nvim-treesitter/nvim-treesitter',
	run = ':TSUpdate'
    }

--NerdTree--
use 'preservim/nerdtree'
use 'ryanoasis/vim-devicons'            --Iconos en NERDtree

--Coc--
use {'neoclide/coc.nvim', branch = 'release'}

--Ale -> Analizar de codigo--
 use 'dense-analysis/ale'

--Auto pairs--
use 'jiangmiao/auto-pairs'

--Lualine status bar--
use {
  'nvim-lualine/lualine.nvim',
  requires = { 'kyazdani42/nvim-web-devicons', opt = true }
}

--Telescope--
use {
  'nvim-telescope/telescope.nvim',
  requires = { {'nvim-lua/plenary.nvim'} }
}

-- Nerd commenter--
use 'preservim/nerdcommenter'

--Debugger para c--
use 'puremourning/vimspector'

--Markdown preview--
use {
    'iamcco/markdown-preview.nvim',
    run = 'cd app && yarn install'
}

--VimTex--
use 'lervag/vimtex'
end)


