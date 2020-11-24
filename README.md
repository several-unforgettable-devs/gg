# GG - Game Jam

If you get stuck at any point,
please drop a message in chat
and we can update this to be more concice.

- [GG - Game Jam](#gg---game-jam)
  - [Reference](#reference)
    - [Rust Book](#rust-book)
  - [Environment Setup](#environment-setup)
    - [git](#git)
    - [rustup](#rustup)
    - [vsc](#vsc)
  - [Final Steps](#final-steps)
    - [Contributing](#contributing)
    - [Debugging](#debugging)

## Reference

### Rust Book

Good to use if you've never worked in rust.

> [https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/)

## Environment Setup

### git

- Install Git (Pick 1) (ignore if you already have git):
  - Install [git for windows](https://gitforwindows.org/)
  - Install Fork
  - Install SourceTree
- Clone this repository
  - Make sure you place the repository on a local drive.
  (USB or networked drives will experience severely degraded rust performance.)
  - Address for Fork or Source Tree: `git@github.com:several-unforgettable-devs/gg.git`
  - Command for cli: `git clone git@github.com:several-unforgettable-devs/gg.git`

### rustup

- Get [rustup](https://www.rust-lang.org/tools/install).
  - Install the 64-BIT version if you're working on windows.
  - You will need to be running on a desktop environment, WSL2 X11 will not work.
- (Optional: Get [Windows Terminal](https://www.microsoft.com/en-us/p/windows-terminal/9n0dx20hk701?activetab=pivot:overviewtab)).
- Launch Powershell.
- Run `rustup update`.

### vsc

- Get [Visual Studio Code](https://code.visualstudio.com/)
- In Powershell:
  - Enter `refreshenv`
  - Navigate to the cloned repository root.
  - Enter `code .`
- Open the extensions menu. (Ctrl + Shift + X).
- Install the recommended extensions.
  - ! If this in unclear, we can clarify. !
- Download and unzip [shaderc](https://github.com/google/shaderc#downloads)
  - Modify .vscode/settings.json to match your path.
  - !! This is annoying for git because it's now dirty. !!
- Enter (Ctrl + Shift + P), type "default shell", set it to powershell.

## Final Steps

### Contributing

- Add yourself to the author section in Cargo.toml.

``` zsh
git add Cargo.toml
git status # Cargo.toml should appear as staged.
git commit -m "Add myself to authors list."
git push

# Failed?
git pull --rebase
git push
```

### Debugging

- In VSC, open the terminal. (Ctrl + `)
- Type `cargo check`
- Type `cargo run`
- F5 should start the debugger.
(If you have created a new cargo.tml file it will ask to generate launch
paramters, accept and submit these if they work.)
