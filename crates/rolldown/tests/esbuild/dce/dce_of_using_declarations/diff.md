# Diff
## /out/entry.js
### esbuild
```js
// entry.js
using null_keep = null;
await using await_null_keep = null;
using throw_keep = {};
using dispose_keep = { [Symbol.dispose]() {
  console.log("side effect");
} };
await using await_asyncDispose_keep = { [Symbol.asyncDispose]() {
  console.log("side effect");
} };
using undef_keep = void 0;
await using await_undef_keep = void 0;
console.log(
  null_keep,
  undef_keep
);
```
### rolldown
```js

```
### diff
```diff
===================================================================
--- esbuild	/out/entry.js
+++ rolldown	
@@ -1,16 +0,0 @@
-// entry.js
-using null_keep = null;
-await using await_null_keep = null;
-using throw_keep = {};
-using dispose_keep = { [Symbol.dispose]() {
-  console.log("side effect");
-} };
-await using await_asyncDispose_keep = { [Symbol.asyncDispose]() {
-  console.log("side effect");
-} };
-using undef_keep = void 0;
-await using await_undef_keep = void 0;
-console.log(
-  null_keep,
-  undef_keep
-);
\ No newline at end of file

```