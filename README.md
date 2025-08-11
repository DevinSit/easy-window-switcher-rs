# Welcome to Easy Window Switcher!

`easy-window-switcher-rs` is a small Rust script/CLI for enabling X window users to more easily change focus between windows that are spread across multiple monitors.

**Upgrade your alt-tab!**

Note: I've only ever tested `easy-window-switcher-rs` in Ubuntu 20.04 under Unity. Your mileage may vary.

Note 2: `easy-window-switcher-rs` is a Rust rewrite of my original Python version, [easy-window-switcher](https://github.com/DevinSit/easy-window-switcher).

## Why did I build Easy Window Switcher?

Because my desktop runs quad monitors and I got so dang tired of having to change between `alt-tab` and `alt-tilde` to target the correct Chrome window.

Not to mention how `alt-tab` handles switching to the last focused window and sometimes focuses onto the wrong window on the wrong monitor.

As such, I decided that it'd be easier to just write a script to be able to **switch focus** to either the **closest left or right window**, or to focus onto the window of a **particular monitor**.

The result is `easy-window-switcher-rs`!

## Dependencies

- [Rust](https://www.rust-lang.org/tools/install) (for building from source)
- `wmctrl` (install using e.g. `sudo apt-get install wmctrl`)
- `xdotool` (install using e.g. `sudo apt-get install xdotool`)
- `xrandr` (install using e.g. `sudo apt-get install x11-xserver-utils` or something, idk anymore)

## Installation

Currently `easy-window-switcher-rs` is only available to be installed from source.

Thankfully, installing from source isn't very hard!

### From Source

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed and run the following: 

```
git clone git@github.com:DevinSit/easy-window-switcher-rs.git
cd easy-window-switcher-rs
make build
```

You can then copy the built binary from `target/release/easy-window-switcher-rs` to wherever you want, like `/usr/local/bin`!

## Usage

`easy-window-switcher-rs` is currently pretty small and, as such, supports just two modes of easy window switching: relative direction and absolute monitor position.

### Relative Direction

Switch focus to the closest left or right window:

```
easy-window-switcher-rs direction left
easy-window-switcher-rs direction right
```

### Absolute Monitor Position

Switch focus to the window on the given monitor (indexed from left-to-right, starting at 0):

```
# Monitor 0 would be the left-most monitor
easy-window-switcher-rs monitor 0

# Monitor 1 would be the monitor to the right of the left-most monitor (i.e. monitor 0)
easy-window-switcher-rs monitor 1
```

### Keyboard Shortcuts

Obviously calling `easy-window-switcher-rs` commands directly from a command line isn't exactly the most optimal way to use it. Binding some preset commands to some keyboard shortcuts is much more effective!

Since I'm a `vim` aficionado, might I suggest `ctrl+super+alt+[h/l]` for `easy-window-switcher-rs direction [left/right]`?

And for the absolute monitor positions, I quite like `ctrl+alt+[1/2/3]` for `easy-window-switcher-rs monitor [0/1/2]`.

But you do you!

### Monitor Configuration

Unlike the original Python version of [easy-window-switcher](https://github.com/DevinSit/easy-window-switcher), `easy-window-switcher-rs` supports automatic monitor configuration out of the box. That's right, no more having to tinker with really janky internal hard-coded configs to get the right number and layout of monitors, it now "just works!" (at least, it does for me)

## Roadmap

There is no roadmap. I might write more tests or tweak things at some point, but otherwise "it works" and this rewrite is a success if I never need to touch it again.

## Contributing

I'm not actively accepting contributions, but feel free to fork it!

## Authors

- **Devin Sit**

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
