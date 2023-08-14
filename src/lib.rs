#![feature(concat_idents, float_next_up_down)]

mod camera;
mod error;
mod layers;
mod ligth_pipeline;
mod math;
mod mouse;
mod objects;
mod scene;
mod scene_manager;
mod shaders;
mod shapes;
mod texture_atlas;
mod wgpu_components;

use error::ErrResult;
use scene_manager::SceneManager;
use smaa::{SmaaMode, SmaaTarget};
use wgpu_components::WgpuContext;
use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

struct State {
    surface: wgpu::Surface,
    context: WgpuContext,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Window,
    smaa_target: SmaaTarget,

    scene_manager: SceneManager,
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

        let context = WgpuContext { device, queue };

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

        surface.configure(&context.device, &config);

        let scene_manager = SceneManager::new(&context, size.width, size.height)?;

        let smaa_target = SmaaTarget::new(
            &context.device,
            &context.queue,
            size.width,
            size.height,
            surface_format,
            SmaaMode::Smaa1X,
        );

        Ok(Self {
            window,
            surface,
            context,
            config,
            size,
            scene_manager,
            smaa_target,
        })
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 && new_size != self.size {
            self.size = new_size;

            self.smaa_target
                .resize(&self.context.device, new_size.width, new_size.height);

            self.scene_manager
                .resize(&self.context, new_size.width, new_size.height);

            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.context.device, &self.config);
        }
    }

    /// Returns true if event is used
    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::Resized(physical_size) => {
                self.resize(*physical_size);
                true
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.resize(**new_inner_size);
                true
            }
            _ => self.scene_manager.handle_event(event),
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let smaa_frame =
            self.smaa_target
                .start_frame(&self.context.device, &self.context.queue, &view);

        self.scene_manager.draw(&self.context, &smaa_frame);

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
            width: 1280,
            height: 720,
        })
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(window).await.unwrap();

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
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
                    _ => {}
                }
            }
        }

        _ => {}
    });
}
