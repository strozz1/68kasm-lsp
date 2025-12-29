# LSP server for 68000 assembly
> [!WARNING]
> The LSP is currently in development, I just implemented the minimal for now.
> 
> Because I made this project before learning anything about compilers, de analysis is not great and makes no sense. After the exams(Jan) I will apply all my new knowledge to improve it!

The **68k LSP** is a LSP server implementation written in Rust for the 68k assembly language for the 68000 Motorola processor.

This assembly language is typically used in Universities and CS courses to teach students about computer architecture and how I/O interruptions work. 
It's normally used with the [bsvc](https://github.com/BSVC/bsvc) simulator.

For this language, files use the `.s` extention, and use 68k asm program for compiling the source code.
```bash
$ 68kasm ./source.s
```
## Server Capabilities
Right now, the server only supports these capabilities:
- **Hover** support
- **Tokenization** support for Highlighting
- Basic **error messages** (still very rudimentary)
- **Go to definition** (only mock up)

## How to Install?
LSP servers are generally used within code editors and IDEs. Usually, you have to manually configure an extension to run an LSP for a specific language.


### Dependencies & requirements
- The server is written in **rust**, so you will need to have it installed on your system. If you don't, you can install it [here](https://www.rust-lang.org/tools/install).



### Manual build
Now that you have `rust` and `cargo` intalled,let's start setting up the server.

First, clone the repo anywhere you want
```bash
git clone https://github.com/strozz1/68kasm-lsp.git
cd ./68kasm-lsp
```
Once you've cloned the repository and navigated into the directory, build the project with `cargo build`.
```bash
cargo build
```
This will generate a **binary file** that will be used to run the server. This is the file your IDE or code editor will be executing in order to communicate with it.

Ideally, you should save this file into a place where you can easilly find and remember.

Make sure to copy the location of the binary, you will need it for the next step.
### Code editor & IDE setup
Now that you have the binary in your sistem, you need to do a little configuration on your code editor/IDE. The process varies between editors.
#### Nvim setup
The nvim setup varies depending on your current nvim configurations and plugins.
If you want to add another plugin setup, make a pull request.

First, you need to make nvim detect `.s` as an asm extension. I'm doing that using the following code:
```lua
-- Set up filetype detection for assembly files
vim.filetype.add({
  extension = {
    s = "asm",      -- Recognize .s files as assembly
    S = "asm",      -- Also handle uppercase .S files
    asm = "asm",
  }
})
```
This is necessary for nvim to recognize the file extension.
##### For nvim 0.11
With the release of nvim 0.11, lsp config in now easier than ever. No need for plugins. 
```lua
-- 68kasm server
vim.lsp.config('kasm_lsp', {
    cmd = { "/home/strozzi/projects/lsp/target/debug/lsp" },
    root_markers = { '.git' },
    filetypes = { 'asm' },
})
vim.lsp.enable('kasm_lsp')
```
##### With nvim-lspconfig
```lua
local lspconfig = require 'lspconfig'
local configs = require 'lspconfig.configs'

if not configs.kasm_lsp then
    configs.kasm_lsp = {
        default_config = {
            cmd = { "your/location/of/the/68kasmlsp binary" },
            root_dir = lspconfig.util.root_pattern('.git'),
            filetypes = { 'asm' },
        },
    }
end
lspconfig.kasm_lsp.setup {}
```
Once your done, open any `.s` assembly files and it should start automatically.

You can check if the server is running by running `:LspInfo`. You should see it there.
#### VSCode setup
> [!NOTE]
> Not yet documented
#### Manual run
> [!NOTE]
> Not yet documented
#### Other editors
Currently, we only have the setup for these editors.
If you wish to add the setup for your favorite editor, feel free to make a pull request.
