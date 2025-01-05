<div align="center">  
  <img src="https://rustacean.net/assets/rustacean-flat-gesture.svg" alt="logo" style="width:120px"></img>
</div>

## Simple Terminal File Manager (STFM)

### Descrption
STFM is a simple terminal-based file manager that lets you navigate through your file system and perform basic file operations.

### Demos

##### Demo Images and Binaries Preview

https://github.com/user-attachments/assets/4cc371d1-d970-4c29-b712-ee6c4a61266a

https://github.com/user-attachments/assets/c8ee1dc7-dc14-4831-90d1-11847145a8b1

### Clone repo
    git clone https://github.com/Chipskein/stfm.git
### Install
    cargo build --release 
### Copy binary to /bin
    cp target/release/stfm /bin/stfm
### Usage
    stfm

### Features
  * Navigate across your file system
  * List file entries in current directory
  * Preview text files
  * Preview binaries in [hexdump](https://en.wikipedia.org/wiki/Hex_dump) format
  * Preview image([See for supported terminal emulators](https://github.com/benjajaja/ratatui-image/blob/master/README.md#compatibility-matrix))
  * Create files/directories in current directory
  * Rename file
  * Search entries by name
  * Copy file
  * Delete file
  * Create Links


### Key controls

- **Navigation:**
  - Use the **arrow keys** to navigate through files.
  - Press **'Enter'** or **'Right Arrow'** to open a file or directory.
  - Press **'Backspace'** or **'Left Arrow'** to go back to the previous directory.

- **Preview:**
  - Select a file to see a preview.
  - With the preview open:
    - Scroll **down** by pressing **'Down Arrow'**.
    - Scroll **up** by pressing **'Up Arrow'**.
    - Scroll **right** by pressing **'Right Arrow'**.
    - Scroll **left** by pressing **'Left Arrow'**.
    - Go back to the main screen by pressing **'q'** or **'Esc'**.

- **Search:**
  - Press **'/'** and type the name of the file to search.
  - With search open, press **'Esc'** to return to the main screen.
  - Press **'Enter'** to show filtered files.

- **File Operations:**
  - Create a new file/directory by pressing **'n'**.
  - Delete a file/directory by pressing **'d'**.
  - Rename a file/directory by pressing **'r'**.
  - Toggle hidden files by pressing **'.'**.
  - Copy a file by pressing **'c'**.
  - Paste a file by pressing **'p'**.

- **Scrolling:**
  - Scroll **down** by pressing **'PageDown'**.
  - Scroll **up** by pressing **'PageUp'**.

- **Exit:**
  - Exit the application by pressing **'q'** or **'Esc'**.

#### TODO MVP
   * [x] Implement Basic file functions(create,rm,list)
   * [x] Implement Rename
   * [x] Fix input block in create file e rename
   * [x] Show/hide hidden Files
   * [x] Add PageDown and PageUp Support
   * [x] Add horizontal Scroll at Preview
   * [x] Add Input Validation in Input Block
   * [x] Create Error PopUp
   * [x] Add Help PopUp
   * [x] Add Different Color Scheme for entries
   * [x] Fix files with wrong Extension
   * [x] Add links support
   * [x] Add Error handling
   * [x] Fix input height bug
   * [x] Display metadata(size,is_dir,modified_at) info in top bar
   * [x] Implement search by entry name 
   * [x] Add Scroll PageUp and Down
   * [x] Implement Copy file function
   * [x] Fix progress bar
   * [x] Add support for preview of non-UTF8 files(binaries)
   * [x] Add support for preview of non-UTF8 files(images)
