import { createRequire } from 'module';
const require = createRequire(import.meta.url);
const dawn = require('../../webgpu/dawn/node-build/dawn.node');

async function delay(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

async function initializeWebGPU() {
  Object.assign(globalThis, dawn.globals);
  const gpuInstance = dawn.create([]);

  // Navigator 설정
  class Navigator {
    get gpu() {
      return gpuInstance;
    }
  }

  // Window와 전역 객체 설정
  const navigator = new Navigator();
  
  globalThis.navigator = navigator;
  globalThis.window = { 
    navigator,
    get gpu() { 
      return gpuInstance; 
    }
  };
  
  globalThis.WorkerGlobalScope = {
    navigator,
    self: globalThis
  };
  
  // global 객체에도 복사
  Object.assign(global, globalThis);
  try {
    const adapter = await gpuInstance.requestAdapter();
    if (!adapter) throw new Error('No adapter found');
    const device = await adapter.requestDevice();
    console.log('GPU Device created:', device);

    const wasm = await import('wasm_example3');
    await wasm.greet();  // greet()가 Promise를 반환한다고 가정
    
    await delay(100);
    device.destroy();
    process.exit(0);
  } catch (error) {
    console.error('Error:', error.stack || error);
    process.exit(1);
  }
}

initializeWebGPU();
