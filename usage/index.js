import init, { chittify } from './pkg/chitter.js';

await init();

const code = document.querySelector('#code');
const highlighted = chittify(`mov rax, '10 is a       string'
add rax, 10`);
code.innerHTML = highlighted;
