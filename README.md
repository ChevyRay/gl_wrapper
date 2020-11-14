# gl_wrapper
A wrapper around https://github.com/brendanzab/gl-rs

Because of the way `gl-rs` loads bindings, intellisense doesn't work when calling into gl functions. This wraps those functions in rust-y wrappers and also error checks after every gl call so we're forced to handle poisoned states.

Most calls into gl are bound-checked for safety, but I might have missed something somewhere, so there are no guarantees here. This is for my personal projects, so maybe just don't use this.