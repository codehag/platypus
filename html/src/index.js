'use strict';
const wasm = require("/Users/Ystartsev/rust-training/platypus/src/main.rs")
window.searchText = () => { console.log("still loading") };

wasm.initialize({noExitRuntime: true}).then(module => {
  const search = module.cwrap('search_text', 'array', ['string', 'string']);
  const dropBytes = module.cwrap('drop_bytes', '', []);

  searchText = (source, search_term, cb) => {
    const vec_ptr = search(source, search_term);
    const ptr = module.HEAPU32[vec_ptr / 4];
    const len = module.HEAPU32[vec_ptr / 4 + 1];
    cb(module.HEAPU8.subarray(ptr, ptr + len));
    dropBytes(ptr);
  }
  searchText(test_source, str, console.log)
})
