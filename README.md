# mcrs

A rust rewrite of [mcpp](https://github.com/rozukke/mcpp), a library to
interface with Minecraft.

Requires a server running [ELCI](https://github.com/rozukke/elci).

```rs
let mut mc = mcrs::Connection::new().unwrap();
mc.post_to_chat("Hello world!").unwrap();
```

