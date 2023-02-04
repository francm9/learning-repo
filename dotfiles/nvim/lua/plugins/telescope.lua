--Configuracion en lua para Telescope--
local status_ok, telescope = pcall(require, "telescope")
if not status_ok then
  return
end

telescope.setup{
    defaults = {
        prompt_prefix = " ",
        selection_caret = " ",
        path_display = { "smart" }
    }

}
