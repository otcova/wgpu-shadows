#![feature(concat_idents)]

mod error;
mod ligth_batch;
mod ligth_pipeline;
mod ligth_shader;
mod quad_batch;
mod quad_shader;
mod texture;
mod texture_atlas;

use error::*;
use ligth_batch::LigthBatch;
use ligth_pipeline::LigthPipeline;
use ligth_shader::LigthShader;
use quad_batch::QuadBatch;
use quad_shader::QuadShader;
use smaa::{SmaaMode, SmaaTarget};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    smaa_target: SmaaTarget,

    ligth_pipeline: LigthPipeline,

    quad_shader: QuadShader,
    quad_batch: QuadBatch,

    ligth_shader: LigthShader,
    ligth_batch: LigthBatch,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: Window) -> ErrResult<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or("Could not get any adapter")?;

        log::info!("Chosen Adapter: {:?}", adapter.get_info());

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = *surface_caps
            .formats
            .iter()
            .find(|f| !f.is_srgb())
            .ok_or("Could not get an srgb surface")?;

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo, // Immediate
            alpha_mode: wgpu::CompositeAlphaMode::Opaque, // surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let ligth_pipeline = LigthPipeline::new(&device, size.width, size.height);
        let ligth_shader = LigthShader::new(&device, &ligth_pipeline.textures, surface_format);
        let ligth_batch = LigthBatch::new(&device);

        let quad_shader = QuadShader::new(&device, &queue)?;
        let quad_batch = QuadBatch::new(&device);

        let smaa_target = SmaaTarget::new(
            &device,
            &queue,
            size.width,
            size.height,
            surface_format,
            SmaaMode::Smaa1X,
        );

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            ligth_pipeline,
            ligth_shader,
            ligth_batch,
            quad_shader,
            quad_batch,
            smaa_target,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;

            self.smaa_target
                .resize(&self.device, new_size.width, new_size.height);

            self.ligth_pipeline
                .resize(&self.device, new_size.width, new_size.height);

            self.ligth_shader
                .resize(&self.device, &self.ligth_pipeline.textures);

            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, _event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
        // todo!()
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let smaa_frame = self
            .smaa_target
            .start_frame(&self.device, &self.queue, &view);

        let mut ligth_frame =
            self.ligth_pipeline
                .start_frame(&self.device, &self.queue, &smaa_frame);

        {
            let mut ligth_pass = ligth_frame.create_render_pass();

            self.quad_shader.bind(&mut ligth_pass);
            self.quad_batch.draw(&mut ligth_pass);

            self.ligth_shader.bind(&mut ligth_pass);
            self.ligth_batch.draw(&mut ligth_pass);
        }

        ligth_frame.resolve();
        smaa_frame.resolve();

        output.present();

        Ok(())
    }
}

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(":)")
        .with_inner_size(LogicalSize {
            width: 640,
            height: 640,
        })
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(window).await.unwrap();

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => log::error!("Render Error: {:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once,
            // unless we manually request it.
            state.window().request_redraw();
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window().id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to dereference it twice
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }

        _ => {}
    });
}
