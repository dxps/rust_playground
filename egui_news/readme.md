# egui based news app

This is a simple example of using [egui](https://github.com/emilk/egui).<br/>
It includes also a text based ui (aka tui).

<br/>

## Prereqs

You may need to install xcb-shape xcb-files libraries in your system,<br/>
otherwise, the compilation fails with such errors:

```
  /usr/bin/ld: cannot find -lxcb-shape
  /usr/bin/ld: cannot find -lxcb-xfixes
```

On Ubuntu based distros, use:`sudo apt install libxcb-shape0-dev libxcb-xfixes0-dev`.
