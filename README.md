# hyprland-relative-workspace

A tool to switch Hyprland workspaces relatively to each monitor and create new workspaces where necessary

Requires Hyprland

## Usage:

### Reccomended

Add binds in your Hyprland config similar to these:
```Properties
bind = $mainMod, left, exec, hyprland-relative-workspace b
bind = $mainMod, right, exec, hyprland-relative-workspace f
bind = $mainMod SHIFT, left, exec, hyprland-relative-workspace b --with-window
bind = $mainMod SHIFT, right, exec, hyprland-relative-workspace f --with-window
```

### Basic command

Use either "f" or "forward" to advance one workspace relative to the current focused workspace (the one with your mouse on it).

Similarily, you can use "b" or "backward" to go back (down, left, eg. workspace 3 -> workspace 2) one workspace relative to the focused monitor. Also creating a new workspace if required and possible (eg. if workspaces 1, and 3 exist, going back one would create workspace 2. However you cannot go lower than 1).

All of these commands can also be used with ``--with-window`` to bring the currently focused window with you.

```
$ hyperland-relative-workspace ["f", "forward", "b", "backward"] (opt. "--with window")
```