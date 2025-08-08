# presto2 

A rust implementation of Presto.

<!--
## Showcase 

![Presto Example 1](docs/example_1.png)
![Presto Example 2](docs/example_2.png)

## Features

### Mostly implemented 
- Entities
- Components
- Cameras
- User scripting
- Events
- Logging and Assertions

### Partially Implemented
- Keyboard Input (Most keycodes aren't available yet other than the arrow keys)
- 3D and 2D rendering (Most PBR properties aren't completed yet, such as normal mapping and non-diffuse textures)
- Debug panel (Custom panels for different component types haven't been implemented yet)

### Planned for the future
- Physics/collision
- Multiple model filetypes supported
- Audio (Including ASIO on Windows)
- Remove GLFW for true multiplatform support (eg. Input, Audio) 
- Live recompilation when modifying scripts and debugging
- Remove need for GLM and smart pointers in dependent applications

## Linking

Since Presto is still in its early stages functionality wise, a formal install system isn't provided. Instead, create your own CMake project, clone or download Presto, then add the following lines to your CMakeLists.txt:

```cmake
add_subdirectory(../Presto ${CMAKE_BINARY_DIR}/Presto) # Presto path here, or leave untouched if it is in an adjacent folder

# You can also disable tests for building Presto
set(PRESTO_ENABLE_TESTS
    OFF
    CACHE BOOL "" FORCE)

# Link PrestoEngine to your game
target_link_libraries([YOUR PROJECT] PRIVATE PrestoEngine)
```


## Example app

Presto's API is exposed through ``Presto/include``. An example application is provided below. Please note that while this demo should work when compiled locally, it is missing the assets required for it to display graphics. As such, please provide assets of your own in place of the textures and models in the code. In future, a demo app will be provided instead.

```cpp
// App.cpp
#include <Presto/EntryPoint.h>
#include <Presto/Presto.h>

#include <Presto/Aliases/All.h>
#include <Presto/Types/All.h>

#include <Presto/Materials.h>
#include <Presto/Objects.h>
#include <Presto/Physics.h>
#include <Presto/Textures.h>

#include "MainCamera.h"

#include "Presto/Assets.h"
#include "Presto/Assets/ImportTypes.h"

namespace Pr = Presto;

class CookOffApp final : public Pr::Application {
   public:
    void setup() override {
        // Set up the default camera
        constexpr auto camera_height = 40;
        Pr::vec3 initial_pos{0, camera_height, 0};
        Pr::GetDefaultCamera()
            .setPosition(initial_pos)
            .setRotation(0, -45.0F, 0)
            .setFOV(40)
            .setDistances({.near = 1, .far = 5000});

        // Load the assets needed for the ground
        ImagePtr ground_image{Pr::LoadImage("assets/textures/ground.png")};

        // You can add assertions to make sure your code is running correctly
        Pr::Assert(ground_image != nullptr, 
                  "The ground image could not be loaded.");
        Presto::Ptr<Presto::Texture2D> ground_texture{
            Pr::NewTexture2D(ground_image)};
        Pr::Assert(ground_texture != nullptr, "Ground texture can't be null.");

        // Create a material for the ground
        auto ground_material{
            Pr::NewMaterial(Pr::MaterialType::DEFAULT_3D, "GroundMaterial")};

        // TODO: Fix the engine to remove the need for this std::dynamic_pointer_cast
        ground_material->setProperty(
            Pr::DefaultMaterialPropertyName::DIFFUSE_TEXTURE,
            std::dynamic_pointer_cast<Pr::Texture>(ground_texture));


        // Create an entity for the new ground, and add its textured quad
        auto e{Pr::NewEntity()};
        Pr::Ptr<RenderComponent> render_component{
            Pr::NewComponent<RenderComponent>()};
        render_component->addQuad({.width = 500,
                                   .height = 500,
                                   .transform{.rotation{0, 90, 0}},
                                   .material{ground_material}});
        e.lock()->setComponent(render_component);

        // MainCamera is a Conductor here, which means its a custom user
        // script which is put on the global main camera
        Pr::SetDefaultCameraConductor(Pr::NewComponent<MainCamera>());

        // Load this glb file as a new mesh source
        // Mesh sources can have multiple meshes inside
        Pr::Ptr<Pr::MeshSource> mesh_source =
            Pr::CreateMeshSource("assets/models/fridge.glb");
        // Load a specific model called "Fridge" from inside of this mesh source
        ModelPtr fridge{mesh_source->loadModel("Fridge")};

        // Add the fridge 3D model to the scene under a new entity
        e = Pr::NewEntity();
        auto rs{Pr::NewComponent<RenderComponent>()};
        rs->addModel(fridge);

        e.lock()->setComponent(rs);
    };

    void gameLoop() override {
        // You can add your own custom logic here
    }

    // Optional
    ~CookOffApp() override = default;
};

// Let Presto know which class overrides it
PRESTO_ENTRY_POINT();
PRESTO_APP_CLASS(CookOffApp);
```

```cpp
// MainCamera.h


// Try to avoid changing script header files, as
// in larger projects this could greatly impact compilation
// time when making changes to the script.

#include <Presto/Presto.h>

#include <Presto/Aliases/All.h>
#include <Presto/Objects.h>
#include <Presto/Types/All.h>

#include <Presto/Scripting.h>

namespace Pr = Presto;

class MainCamera : public Conductor {
   public:
    void setFollow(const Pr::Ptr<Transform>&);
    void setOffset(const Pr::vec3&);

   private:
    // void on(Presto::KeyEvent& e) override;

    void start() override;
    void update() override;

    bool up_{false};
    bool down_{false};
    bool left_{false};
    bool right_{false};

    Pr::ComponentPtr<Camera> camera_;

    Pr::EventListener listener;
};

-->
