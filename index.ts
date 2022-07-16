import { dlopen, suffix, FFIType, CString } from "bun:ffi";

// TODO: figure out how to have a string as an argument
const {
  symbols: { hash },
} = dlopen(`./target/debug/deps/libhashing_example.${suffix}`, {
  hash: {
    returns: "u64",
  },
});

const computedHash = hash();

console.log("Hash:", computedHash);
