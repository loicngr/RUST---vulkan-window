use vulkano_win::{
    VkSurfaceBuild,
};
use vulkano::{
    instance::{Instance, ApplicationInfo, Version}
};
use winit::{
    event_loop::{
        EventLoop,
        ControlFlow
    },
    event::{Event, WindowEvent},
    window::WindowBuilder,
};
use std::sync::Arc;

// Création de l'instance
fn vulkan_make_instance() -> Arc<Instance> {
    let app_info = ApplicationInfo {
        application_name: Some("Ma fenêtre !".into()),
        application_version: Some(Version { major: 1, minor: 0, patch: 0 }),
        engine_name: Some("No Engine".into()),
        engine_version: Some(Version { major: 1, minor: 0, patch: 0 }),
    };

    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(Some(&app_info), &extensions, None).expect("failed to create Vulkan instance")
    };

    instance
}

fn main() {
    let events_loop = EventLoop::new();
    let instance = vulkan_make_instance();

    let _surface = WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();

    events_loop.run(|event, _, control_flow| {

        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            _ => ()
        }
    });
}
