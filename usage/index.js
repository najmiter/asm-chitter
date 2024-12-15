import init, { chittify } from '../pkg/chitter.js';

await init();

const code = document.querySelector('#code');
code.innerHTML = chittify(`mov rax, "10 is a string"
add rax, 10`);
