# HTML WASM Sprinkles

I'd like to investigate the ability to sprinkle HTML pages with WASM instead of
JS.

Sometimes it makes sense to repeat part of calculations in the browser to avoid
a round trip to the server. So it would be great to split it into a library and
use it on both sides to ensure the calculations are always in sync.

## Run
```sh
./run.sh
```

## Examples

- `basic` - (112 bytes) the simplest example without any dependencies
- `basic-bindgen` - (142 bytes + js glue) the previous example using `wasm_bindgen`

## File sizes

| example       | WASM  | JS glue |
|---------------|-------|---------|
| basic         | 112 B | -       |
| basic-bindgen | 142 B | 2.5 K   |


