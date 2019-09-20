use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::device::{Device, DeviceExtensions};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::swapchain::{AcquireError, PresentMode, SurfaceTransform, Swapchain, SwapchainCreationError};

use vulkano_win::VkSurfaceBuild;
use winit::{EventsLoop, Window, WindowBuilder, Event, WindowEvent};

fn main() {
    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).unwrap()
    };

    // Use the first device that is found, which should work in most cases.
    let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
    println!("Using device {} (type: {:?})", physical.name(), physical.ty());

    // Create a window which also contains a Vulkan surface.
    let mut events_loop = EventsLoop::new();
    let surface = WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();
    let window = surface.window();

    // Choose which queue to run on, which are similar to CPU queues in that
    // they can run in parallel. There are usually a graphics queue, transfer
    // queue and a compute queue.
    let queue_family = physical.queue_families().find(|&q| {
        // Take the first queue that supports drawing to the window.
        q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
    }).unwrap();

    let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };
    let (device, mut queues) = Device::new(physical, physical.supported_features(), &device_ext,
                                           [(queue_family, 0.5)].iter().cloned()).unwrap();
    let queue = queues.next().unwrap();

    // We now have to create a swapchain which will allocate the colour buffers
    // that eventually contain the image that is rendered.
    let (mut swapchain, images) = {
        // Query the capabilities of the surface.
        let caps = surface.capabilities(physical).unwrap();
        let usage = caps.supported_usage_flags;

        // Set the alpha mode of the window which indicates how alpha values of
        // the final image will behave.
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();

        // Choosing the internal format that the images will have.
        let format = caps.supported_formats[0].0;

        // Set the dimensionos of the window. Some will specity a
        // caps.current_extent, whereas others will not set that value.
        let initial_dimensions = if let Some(dimensions) = window.get_inner_size() {
            // Convert to physical pixels.
            let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
            [dimensions.0, dimensions.1]
        } else {
            return;
        };

        Swapchain::new(device.clone(), surface.clone(), caps.min_image_count, format,
                       initial_dimensions, 1, usage, &queue, SurfaceTransform::Identity,
                       alpha, PresentMode::Fifo, true, None).unwrap()
    };

    // Create the buffer that will store the shape of the triangle.
    let vertex_buffer = {
        #[derive(Default, Debug, Clone)]
        struct Vertex { position: [f32; 2] }
        vulkano::impl_vertex!(Vertex, position);

        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), [
            Vertex { position: [-0.5, -0.25] },
            Vertex { position: [0.0, 0.5] },
            Vertex { position: [0.25, -0.1] }
        ].iter().cloned()).unwrap()
    };

    // We then have to create the shaders.
    
}
