use std::sync::Arc;

use vulkano::{
    VulkanLibrary,
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo,
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
    },
    device::{
        self, Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags, physical::PhysicalDevice,
    },
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{
        AllocationCreateInfo, FreeListAllocator, GenericMemoryAllocator, MemoryTypeFilter,
        StandardMemoryAllocator,
    },
    sync::{self, GpuFuture},
};

pub(super) struct VulkanContext {
    // Instances
    library: Arc<VulkanLibrary>,
    instance: Arc<Instance>,
    physical_device: Arc<PhysicalDevice>,
    device: Arc<Device>,
    // Queues
    graphics_queue: Arc<device::Queue>,
    // Allocators
    memory_allocator: Arc<GenericMemoryAllocator<FreeListAllocator>>,
    command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
}

struct VulkanRenderer {
    vk: VulkanContext,
}

impl VulkanRenderer {
    pub fn create() -> Result<Box<VulkanRenderer>, Box<dyn std::error::Error>> {
        let library = VulkanLibrary::new()?;

        let instance = Instance::new(
            library.clone(),
            InstanceCreateInfo::application_from_cargo_toml(),
        )?;

        // Get the first Vulkan capable device
        let physical_device = instance
            .enumerate_physical_devices()
            .expect("could not enumerate devices")
            .next()
            .expect("no devices available");

        for family in physical_device.queue_family_properties() {
            println!(
                "Found a queue family with {:?} queue(s)",
                family.queue_count
            );
        }

        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_queue_family_index, queue_family_properties)| {
                queue_family_properties
                    .queue_flags
                    .contains(QueueFlags::GRAPHICS)
            })
            .expect("couldn't find a graphical queue family")
            as u32;

        let (device, mut queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                // here we pass the desired queue family to use by index
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("failed to create device");

        // Assumed only a single queue, the graphics queue
        let queue = queues.next().unwrap();

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        ));

        Ok(Box::new(VulkanRenderer {
            vk: VulkanContext {
                library,
                instance,
                physical_device,
                device,
                graphics_queue: queue,
                memory_allocator,
                command_buffer_allocator,
            },
        }))
    }
}
/*
*
       let data: i32 = 12;
       let buffer = Buffer::from_data(
           memory_allocator.clone(),
           BufferCreateInfo {
               usage: BufferUsage::UNIFORM_BUFFER,
               ..Default::default()
           },
           AllocationCreateInfo {
               memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                   | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
               ..Default::default()
           },
           data,
       )
       .expect("failed to create buffer");

       // Staging buffer for transferring data
       let source_content: Vec<i32> = (0..64).collect();
       let staging_buffer = Buffer::from_iter(
           memory_allocator.clone(),
           BufferCreateInfo {
               usage: BufferUsage::TRANSFER_SRC,
               ..Default::default()
           },
           AllocationCreateInfo {
               memory_type_filter: MemoryTypeFilter::PREFER_HOST
                   | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
               ..Default::default()
           },
           source_content,
       )
       .expect("failed to create source buffer");

       let destination = Buffer::from_iter(
           memory_allocator.clone(),
           BufferCreateInfo {
               usage: BufferUsage::TRANSFER_DST,
               ..Default::default()
           },
           AllocationCreateInfo {
               memory_type_filter: MemoryTypeFilter::PREFER_HOST
                   | MemoryTypeFilter::HOST_RANDOM_ACCESS,
               ..Default::default()
           },
           (0..64).map(|_| 0).collect(),
       )
       .expect("failed to create destination buffer");

        let mut builder = AutoCommandBufferBuilder::primary(
            command_buffer_allocator.clone(),
            queue_family_index,
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .copy_buffer(CopyBufferInfo::buffers(
                staging_buffer.clone(),
                destination.clone(),
            ))
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let future = sync::now(device.clone())
            .then_execute(queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush() // same as signal fence, and then flush
            .unwrap();

        let src_content = staging_buffer.read().unwrap();
        let destination_content = destination.read().unwrap();
        assert_eq!(&*src_content, &*destination_content);

        println!("Everything succeeded!");

        future.wait(None).unwrap();


*/
