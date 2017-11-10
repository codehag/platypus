'use strict';
const wasm = require("../../src/main.rs")
window.get_data = () => { console.log("still loading") };

wasm.initialize({noExitRuntime: true}).then(module => {
  var str = "Hello from JS.";
  get_data = module.cwrap('get_data', 'string', ['string']);
  console.log(get_data(str));
  // you can call module.cwrap here to get function wrappers for Rust functions
})

