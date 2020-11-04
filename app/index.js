async function loadWasm() {
  await WebAssembly.instantiateStreaming(fetch("./utils/utils.gc.wasm")).then(
    (wasmModule) => {
      const result = wasmModule.instance.exports.add_one(10);
      const text = document.createTextNode(result);
      document.body.appendChild(text);
      window.add_one = wasmModule.instance.exports.add_one;
    }
  );
}

(async () => {
    const response = await fetch('./utils/utils.gc.wasm');
    const buffer = await response.arrayBuffer();
    const module = await WebAssembly.compile(buffer);
    const instance = new WebAssembly.Instance(module);
    const result = instance.exports.add_one(10);
    const text = document.createTextNode(result);
    document.body.appendChild(text);
    window.add_one = instance.exports.add_one;
    console.log(window.add_one(400))
  })();


window.add_one(20);