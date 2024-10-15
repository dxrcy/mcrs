# mcrs

A rust rewrite of [mcpp](https://github.com/rozukke/mcpp), a library to
interface with Minecraft.

```rs
let mut mc = mcrs::Connection::new().unwrap();
mc.post_to_chat("Hello world!").unwrap();
```

<details>
<summary>TODO</summary>

- Update readme
- Add tests

</details>

