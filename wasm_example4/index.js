import { createRequire } from 'module';
const require = createRequire(import.meta.url);
const dawn = require('../../webgpu/dawn/node-build/dawn.node');

async function delay(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

async function initializeWebGPU() {
  Object.assign(globalThis, dawn.globals);
  const gpuInstance = dawn.create([]);

  class Navigator {
    get gpu() {
      return gpuInstance;
    }
  }

  const navigator = new Navigator();
  
  globalThis.navigator = navigator;
  globalThis.Window = { 
    navigator,
  };
  
  try {
    const adapter = await gpuInstance.requestAdapter();
    if (!adapter) throw new Error('No adapter found');
    const device = await adapter.requestDevice();
    console.log('GPU Device created:', device);

    const wasm = await import('wasm_example3');
    await wasm.greet();
    
    await delay(100);
    device.destroy();
    process.exit(0);
  } catch (error) {
    console.error('Error:', error.stack || error);
    process.exit(1);
  }
}

initializeWebGPU();
