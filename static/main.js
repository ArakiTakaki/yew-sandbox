import init, { run_app } from './pkg/yew_exmaple.js';

async function main() {
   await init('./pkg/yew_exmaple_bg.wasm');
   run_app();
}
main();
