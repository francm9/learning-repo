-- Configuracion para treesiter --
require'nvim-treesitter.configs'.setup {
  -- Highlighting -- 
  highlight = {
    enable = true,
    custom_captures = {
      -- Highlight the @foo.bar capture group with the "Identifier" highlight group.
      ["foo.bar"] = "Identifier",
    },
    -- Setting this to true will run `:h syntax` and tree-sitter at the same time.
    -- Set this to `true` if you depend on 'syntax' being enabled (like for indentation).
    -- Using this option may slow down your editor, and you may see some duplicate highlights.
    -- Instead of true it can also be a list of languages
    additional_vim_regex_highlighting = false,
  },
    -- Identaciones --
    indent = {
        enable = true
    },
    -- Rainbow Brackets --
    rainbow = {
        enable = true,
        extended_mode = true,
        max_file_lines = nil,
    }
}

