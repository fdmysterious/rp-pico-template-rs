Rust rp-pico template
=====================

- Florian Dupeyron
- June 2023

This minimal template is inspired by https://github.com/rp-rs/rp2040-project-template.

The following elements are added:

- dockerized environment for easy installation
- just is used for various shortcuts commands

# Dependencies

- docker
- just (`cargo install just`)

# Building

Just as simple as:

```c
just build
```

# TODO

- [ ] Debugging using `picoprobe`
- [ ] Add option to `build` target to choose build type (release or debug)
- [ ] Various improvements to the `just` file
- [ ] Add vscode config files using devcontainer, _etc._
