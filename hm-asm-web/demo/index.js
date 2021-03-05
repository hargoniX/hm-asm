import * as wasm from "hm-asm-web";


console.log("Welcome to hm-asm-web");
const code = "LDA #1\nADD #3\nSTA (8)";
console.log("Compiling:\n" + code);
console.log(wasm.assemble(code));
console.log("Simulating for four clock cycles:\n" + code);
console.log(wasm.simulate(code, 4));

const invalid = "LDA dkfljlsdkfjsdf";
console.log("Compiling invalid code:\n" + invalid);
console.log(wasm.assemble(invalid));
console.log("Hurray a nice error msg");
