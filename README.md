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

- `basic` - the simplest example without any dependencies
- `basic-bindgen` - the previous example using `wasm_bindgen`
- `greet` - working with strings which aren't native to WASM
- `events` - interactive greeting using `web_sys`

## Bundle sizes

| example       | WASM  | JS glue |
|---------------|-------|---------|
| basic         | 112 B | -       |
| basic-bindgen | 142 B | 2.5 K   |
| greet         | 9.7 K | 5.5 K   |
| events        | 19 K  | 14 K    |
