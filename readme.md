# Clipboard-RS

> Node binding for rust [clipboard-rs] lib, powered by [napi-rs].

Support read/write text/image/filePath to clipboard, cross-platform.

## Usage

```sh
npm i clipboard-rs
# or
yarn i clipboard-rs
# or
pnpm i clipboard-rs
```

## Usage

```ts
import { readText, readFiles, readImage, writeText, writeImage, writeFiles } from 'clipboard-rs'

const text = readText() // => string | null
const imageBuffer = readImage() // => Buffer | null
const files = readFiles() // => string[] | null

// return true means write to clipboard successfully
writeText('text content') // => true | null
writeImage(buf) // => true | null
writeFiles([path]) // => true | null
```

[clipboard-rs]: https://github.com/ChurchTao/clipboard-rs
[napi-rs]: https://github.com/napi-rs/napi-rs
