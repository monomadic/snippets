# Cargo Tricks

## Patch Local Deps
``` toml
[patch."https://github.com/servo/webrender"]
"webrender" = { path = "<path>/webrender" }
"webrender_api" = { path = "<path>/webrender_api" }
```
