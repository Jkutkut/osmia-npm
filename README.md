# osmia npm:

A simple npm package to run [osmia](https://github.com/jkutkut/osmia). This project compiles directly to WebAssembly and enables you to run osmia natively as a normal npm package.

## Usage:

### As a node pkg-module:

Add the following to your `package.json`:

```json
{
    "dependencies": {
        "osmia-npm": "github:jkutkut/osmia-npm#<version>"
    }
}
```

Note: You can have a look at [osmia-vscode](https://github.com/jkutkut/osmia-vscode) as an example of how to use osmia in TS.

Note: `<version>` refers to a stable tag in this repository. For example: `v0.4.0`.

Note: You can compile the project directly with the use of the Makefile.

### As a web module - pkg module:

Live example: [osmia-npm pkg module example](https://jkutkut.github.io/osmia-npm/)

```js
import('/pkg/osmia_npm.js').then(async (osmia) => {
    // TODO use osmia-npm API here
});
```

Note: You have a small demo if you: Compile the pkg module with `make build_dev` and preview the [index.html](./index.html) file.

## Docs:

Compile the docs directly with `make doc` and preview the `./target/doc/osmia_npm/index.html` file.

You may view it directly [on this branch](https://github.com/Jkutkut/osmia-npm/tree/documentation), opening the file `./osmia_npm/index.html`.
