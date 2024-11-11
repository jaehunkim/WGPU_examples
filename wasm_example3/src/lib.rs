mod execute_gpu;

use crate::execute_gpu::run;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// pub async fn run() {
//     let my_shader = "
//         @group(0) @binding(0) var<uniform> coefficient: u32;
//         @group(0) @binding(1) var<storage, read> in: array<u32>;
//         @group(0) @binding(2) var<storage, read_write> out: array<u32>;
//         @compute
//         @workgroup_size(8, 1, 1)
//         fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
//             out[global_id.x] = coefficient * in[global_id.x];
//         }
//     ";

//     let instance = wgpu::Instance::default();
//     let adapter = instance
//         .request_adapter(&wgpu::RequestAdapterOptions {
//             power_preference: wgpu::PowerPreference::HighPerformance,
//             compatible_surface: None,
//             force_fallback_adapter: false,
//         })
//         .await
//         .expect("GPU not available.");

//     let (device, queue) = adapter
//         .request_device(
//             &wgpu::DeviceDescriptor {
//                 label: None,
//                 required_features: wgpu::Features::TIMESTAMP_QUERY,
//                 required_limits: wgpu::Limits::downlevel_defaults(),
//                 memory_hints: wgpu::MemoryHints::MemoryUsage,
//             },
//             None,
//         )
//         .await
//         .unwrap();

// }

#[wasm_bindgen]
pub fn greet() {
    //alert("Hello, wasm_example4!");
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run());
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run());
    }
    // alert("Hello, wasm_example5!");
    // let mut pipeline = gpu.gen_pipeline(
    //     None,
    //     [StageDesc {
    //         name: Some("norm"),
    //         shader: my_shader,
    //         entrypoint: "main",
    //     }],
    // );

    // const COEFFICIENT: u32 = 42;
    // let input: [u32; N_ELEMENT] = std::array::from_fn(|i| i as u32);
    // pipeline.write_uniform(&COEFFICIENT);
    // let result_gpu = pipeline.run(&input, [(N_WORKGROUP, 1, 1)], |vals: &[u32; N_ELEMENT]| {
    //     *vals
    // });
    // let result_cpu = input.map(|v| v * COEFFICIENT);
    // assert_eq!(result_gpu, result_cpu);

    // let alert_msg = format!("{:?}", result_gpu);
    // alert(&alert_msg);
}
