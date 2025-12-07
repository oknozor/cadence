-- Local nvim config for cadence project
-- Auto-format Rust files with dx format on save

vim.api.nvim_create_autocmd("BufWritePost", {
  pattern = "*.rs",
  callback = function()
    local filename = vim.api.nvim_buf_get_name(0)
    vim.cmd("silent !dx fmt")
    vim.cmd("edit!")
  end,
})
