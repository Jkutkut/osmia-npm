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

```js
import('/pkg/osmia_npm.js').then(async (osmia) => {
    // TODO use osmia-npm API here
});
```

Note: You have a small demo if you: Compile the pkg module with `make build_dev` and preview the [index.html](./index.html) file.
