'use strict';
const wasm = require("../../src/main.rs")
window.getDataWrapper = () => { console.log("still loading") };

wasm.initialize({noExitRuntime: true}).then(module => {
  const str = "some";
  const getData = module.cwrap('get_data', 'array', ['string']);
  const dropBytes = module.cwrap('drop_bytes', '', []);
  getDataWrapper = (search, cb) => {
    const vec_ptr = getData(search);
    const ptr = module.HEAPU32[vec_ptr / 4];
    const len = module.HEAPU32[vec_ptr / 4 + 1];
    cb(module.HEAPU8.subarray(ptr, ptr + len));
    dropBytes(ptr);
  }
  getDataWrapper(str, console.log)
})

