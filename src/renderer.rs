use vulkano::instance::{Instance, InstanceCreateInfo};

pub(crate) struct VulkanRenderer {}

impl VulkanRenderer {
    pub fn create() -> Result<(), Box<dyn std::error::Error>> {
        let vk_lib = vulkano::VulkanLibrary::new()?;

        let instance = Instance::new(vk_lib, InstanceCreateInfo::application_from_cargo_toml())?;
        Ok(())
    }
}
