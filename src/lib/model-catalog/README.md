# Model catalog library (vendored)

TypeScript sources of `@handy-computer/models` 0.2.2, copied from
`handy-computer/models` commit `23e87bb6f03c6d75f971e24a78eb2c548a50b395`
(`packages/models/src/`) so the app no longer depends on the npm package. The
only local change: `index.ts` reads the app's own catalog at
`src-tauri/src/catalog/catalog.json`, which the Rust side compiles in too, so
both sides share one copy. Edit these files here; nothing syncs them.

The package is published under the MIT License:

Copyright (c) the @handy-computer/models authors

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
