'use strict';
const wasm = require("../../src/main.rs")
window.getData = () => { console.log("still loading") };

wasm.initialize({noExitRuntime: true}).then(module => {
  const str = "some";
  getData = module.cwrap('get_data', 'string', ['string']);
  console.log(getData(str));
})

