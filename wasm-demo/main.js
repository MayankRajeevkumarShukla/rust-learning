import init, { greet } from './pkg/wasm_demo.js';

async function run() {
  // Initialize the wasm module
  await init();

  // Now you can call the greet function
  const result = greet("World");
  console.log(result); // Should print: "Hello, World from WebAssembly!"
  
  // Update the page
  document.body.innerHTML += `<p>${result}</p>`;
}

run();