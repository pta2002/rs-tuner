# TuneRS

TuneRS is a command line instrument tuner written in rust, inspired by
[tuneit](https://delysid.org/tuneit.html), but aiming to have a nicer user
interface.

Here's what it currently looks like:

![the TuneRS interface](https://raw.githubusercontent.com/pta2002/tuners/master/screenshot.png)

# Usage

Right now, TuneRS requires [JACK](https://jackaudio.org/), but support for other
audio systems is planned.

To install TuneRS, use cargo:

```bash
cargo install --git https://github.com/pta2002/tuners.git
```

Then just run it with bash. To quit, press Ctrl+C.
