use std::collections::HashMap;
use std::path::PathBuf;

use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

use gltf_json as gltf;

/// Export entire Blender file to glTF
pub fn export(py: Python, export_path: &str) -> PyResult<()> {
    // Create an exporter
    let mut exporter = BlendExporter::new(export_path);

    // Export the blend
    exporter.export(py)?;

    Ok(())
}

/// A set of HashMaps that maps Blender object names to their corresponding
/// index in the glTF data.
#[derive(Default)]
struct DataNames {
    scene_names: HashMap<String, u32>,
    object_names: HashMap<String, u32>,
    mesh_names: HashMap<String, u32>,
}

/// Data stored for vertices in the glTF mesh buffer. Has a "C" repr so that its
/// memory representation is reproducible when used with the
/// `to_padded_byte_vector()` helper function.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct VertexData {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

/// Helper function used to convert a vector to a byte representation. The type
/// contained in the vector should be repr(C) so that its byte representation is
/// reproducible.
///
/// Taken straight from here:
/// https://github.com/gltf-rs/gltf/blob/75df52c9bb849428262968b77d6d41c9d9709a10/examples/export/main.rs#L29
#[cfg_attr(feature = "enable_profiling", flame)]
fn to_padded_byte_vector<T>(vec: Vec<T>) -> Vec<u8> {
    let byte_length = vec.len() * std::mem::size_of::<T>();
    let byte_capacity = vec.capacity() * std::mem::size_of::<T>();
    let alloc = vec.into_boxed_slice();
    let ptr = Box::<[T]>::into_raw(alloc) as *mut u8;
    let mut new_vec = unsafe { Vec::from_raw_parts(ptr, byte_length, byte_capacity) };
    while new_vec.len() % 4 != 0 {
        new_vec.push(0); // pad to multiple of four bytes
    }
    new_vec
}

/// The SceneExporter
struct BlendExporter {
    gltf_doc: gltf::Root,
    data_names: DataNames,
    export_dir: PathBuf,
}

impl BlendExporter {
    /// Instantiate a scene exporter by providing the export dir
    pub fn new(export_dir: &str) -> Self {
        BlendExporter {
            gltf_doc: Default::default(),
            data_names: Default::default(),
            export_dir: PathBuf::from(export_dir),
        }
    }

    /// Helper function to write resource data to disk
    #[cfg_attr(feature = "enable_profiling", flame)]
    fn write_resource(&self, file_name: &str, bytes: &[u8]) -> PyResult<()> {
        std::fs::write(&self.export_dir.join(file_name), bytes)?;

        Ok(())
    }

    /// Export entire Blender file to glTF
    #[cfg_attr(feature = "enable_profiling", flame)]
    pub fn export(&mut self, py: Python) -> PyResult<()> {
        // Blender data import
        let data = py.import("bpy")?.get("data")?.to_object(py);

        // Add a default material
        self.gltf_doc.materials.push(gltf::Material::default());

        // For every Blender scene
        let scenes = data.getattr(py, "scenes")?.call_method0(py, "values")?;
        for scene in scenes.cast_as::<PyList>(py)? {
            let scene = scene.to_object(py);

            // Load scene
            self.load_scene(py, scene)?;
        }

        // Write glTF file to disk
        self.write_resource(
            "export.gltf",
            gltf::serialize::to_string_pretty(&self.gltf_doc)
                .unwrap()
                .as_bytes(),
        )?;

        // Dump the static assets. These will eventually be dynamically
        // generated based on the Blend configuration.
        self.write_resource("scene.ron", include_bytes!("exporter/scene.ron"))?;
        self.write_resource("display_config.ron", include_bytes!("exporter/display_config.ron"))?;

        Ok(())
    }

    /// Load scene data into `gltf_doc` and `data_names` and return scene ID
    #[cfg_attr(feature = "enable_profiling", flame)]
    fn load_scene(&mut self, py: Python, scene: PyObject) -> PyResult<u32> {
        // Get scene name
        let scene_name: String = scene.getattr(py, "name")?.extract(py)?;

        // If the scene has already been loaded
        if let Some(scene_id) = self.data_names.scene_names.get(&scene_name) {
            // Skip loading and return scene id
            return Ok(*scene_id);
        }

        // Objects in the scene
        let objects = scene.getattr(py, "objects")?.call_method0(py, "values")?;
        // Scene nodes
        let mut nodes = vec![];

        // For each object
        for object in objects.cast_as::<PyList>(py)? {
            let object = object.to_object(py);

            // Skip objects with parents. We only load root objects in the scene.
            if object.getattr(py, "parent")? != py.None() {
                continue;
            }

            // Add node to scene
            nodes.push(gltf::Index::new(self.load_object(py, object)?));
        }

        // Get the scene index
        let scene_index = self.gltf_doc.scenes.len() as u32;

        // Add scene to name list
        self.data_names
            .scene_names
            .insert(scene_name.clone(), scene_index);

        // Add scene to glTF
        self.gltf_doc.scenes.push(gltf::Scene {
            name: Some(scene_name),
            nodes,
            extras: None,
            extensions: None,
        });

        // Return scene index
        Ok(scene_index)
    }

    /// Load an object and return its integer id
    #[cfg_attr(feature = "enable_profiling", flame)]
    fn load_object(&mut self, py: Python, object: PyObject) -> PyResult<u32> {
        // Object name
        let object_name: String = object.getattr(py, "name")?.extract(py)?;

        // If the object has already been loaded
        if let Some(object_id) = self.data_names.object_names.get(&object_name) {
            // Return object ID
            return Ok(*object_id);
        }

        // Blender child objects
        let blend_children = object.getattr(py, "children")?;

        // glTF children nodes
        let mut gltf_children = vec![];

        // For each child object
        for child in blend_children.cast_as::<PyTuple>(py)? {
            let child = child.to_object(py);

            gltf_children.push(gltf::Index::new(self.load_object(py, child)?));
        }

        // Initialize object types as None
        let mut mesh = None;
        let camera = None;

        // The type of Blender object
        let object_type: String = object.getattr(py, "type")?.extract(py)?;

        // If the object is a mesh
        if object_type == "MESH" {
            // Get Blender mesh
            let blender_mesh = object.call_method0(py, "to_mesh")?;

            // Load mesh data
            mesh = Some(gltf::Index::new(self.load_mesh(py, blender_mesh)?));

        // If the object is a camera
        } else if object_type == "CAMERA" {
            // TODO: Handle camera objects
        }

        // Object translation
        let mut translation = [0f32; 3];
        for (i, coordinate) in translation.iter_mut().enumerate() {
            *coordinate = object
                .getattr(py, "location")?
                .call_method1(py, "__getitem__", PyTuple::new(py, &[i]))?
                .extract(py)?;
        }
        // Change coordinates so that Y is up
        translation = [translation[0], translation[2], -translation[1]];

        // Object rotation
        let mut rotation = [0f32; 4];
        // Get quaternion representation of Blender rotation
        let blender_quat;
        if object.getattr(py, "rotation_mode")?.extract::<String>(py)? == "QUATERNION" {
            blender_quat = object.getattr(py, "rotation_quaternion")?;
        } else {
            blender_quat = object.
                getattr(py, "rotation_euler")?
                .call_method0(py, "to_quaternion")?;
        }
        // Collect coordinates from blender quat
        for (i, coordinate) in rotation.iter_mut().enumerate() {
            *coordinate = blender_quat
                .call_method1(py, "__getitem__", PyTuple::new(py, &[i]))?
                .extract(py)?;
        }
        // Change coordinates so that Y is up
        rotation = [rotation[1], rotation[3], -rotation[2], rotation[0]];

        // Object Scale
        let mut scale = [0f32; 3];
        for (i, coordinate) in scale.iter_mut().enumerate() {
            *coordinate = object
                .getattr(py, "scale")?
                .call_method1(py, "__getitem__", PyTuple::new(py, &[i]))?
                .extract(py)?;
        }
        // Change coordinates so that Y is up
        scale = [scale[0], scale[2], scale[1]];

        // The index of the object in the glTF document
        let node_index = self.gltf_doc.nodes.len() as u32;

        // Add object name to the names index
        self.data_names
            .object_names
            .insert(object_name.clone(), node_index);

        // Add object to the glTF node list
        self.gltf_doc.nodes.push(gltf::Node {
            name: Some(object_name),
            children: {
                if gltf_children.is_empty() {
                    None
                } else {
                    Some(gltf_children)
                }
            },
            translation: Some(translation),
            rotation: Some(gltf::scene::UnitQuaternion(rotation)),
            scale: Some(scale),
            mesh,
            camera,
            matrix: None,
            skin: None,
            weights: None,
            extras: None,
            extensions: None,
        });

        // Return the node index
        Ok(node_index)
    }

    /// Load a mesh and return the integer id
    #[cfg_attr(feature = "enable_profiling", flame)]
    fn load_mesh(&mut self, py: Python, mesh: PyObject) -> PyResult<u32> {
        // Get the mesh name
        let mesh_name: String = mesh.getattr(py, "name")?.extract(py)?;

        // If this mesh has already been loaded
        if let Some(mesh_id) = self.data_names.mesh_names.get(&mesh_name) {
            // Return mesh ID
            return Ok(*mesh_id);
        }

        // The binary buffer that will store the mesh data
        let mut mesh_buffer: Vec<u8>;
        // The number of indices
        let indices_count;
        // The byte length of indices data
        let indices_length;
        // The number of vertices
        let vertices_count;
        // The byte length of vertices data
        let vertices_length;
        // The minium vertex position
        let mut vertex_min = [0f32; 3];
        // The maximum vertex position
        let mut vertex_max = [0f32; 3];

        // Add indices to mesh buffer
        {
            #[cfg(feature = "enable_profiling")]
            let _guard = flame::start_guard("get_mesh_indices");

            // Calculate mesh tesselation
            mesh.call_method0(py, "calc_loop_triangles")?;

            // Collect the triangle data
            let loop_tris = mesh
                .getattr(py, "loop_triangles")?
                .call_method0(py, "values")?;
            let loop_tris = loop_tris.cast_as::<PyList>(py)?;

            // Mesh index data
            let mut mesh_indices: Vec<i16> = Vec::with_capacity(loop_tris.len());

            // For every triangle
            for tri in loop_tris {
                // Get the vertice indices
                let verts = tri.to_object(py).getattr(py, "vertices")?;

                // Add the indices to the mesh data
                for i in 0..3 {
                    mesh_indices.push(
                        verts
                            .call_method1(py, "__getitem__", PyTuple::new(py, &[i]))?
                            .extract(py)?,
                    );
                }
            }

            // Add indices to the buffer
            indices_count = mesh_indices.len();
            mesh_buffer = to_padded_byte_vector(mesh_indices);
            indices_length = mesh_buffer.len();
        }

        // Add vertices to mesh buffer
        {
            #[cfg(feature = "enable_profiling")]
            let _guard = flame::start_guard("get_mesh_vertices");

            // Collect mesh vertices
            let verts = mesh.getattr(py, "vertices")?.call_method0(py, "values")?;
            let verts = verts.cast_as::<PyList>(py)?;

            // Mesh vertice data
            let mut mesh_vertices: Vec<VertexData> = Vec::with_capacity(verts.len());

            // For every vertice
            for vert in verts {
                // Vertice cooridinates
                let co = vert.to_object(py).getattr(py, "co")?;
                // Vertice normal
                let normal = vert.to_object(py).getattr(py, "normal")?;

                // Extract vertex position and normal. Y and Z axes are swaped and
                // inverted to switch Y to the up axis.
                let vertex = VertexData {
                    position: [
                        co.getattr(py, "x")?.extract(py)?,
                        co.getattr(py, "z")?.extract(py)?,
                        -co.getattr(py, "y")?.extract(py)?,
                    ],
                    normal: [
                        normal.getattr(py, "x")?.extract(py)?,
                        normal.getattr(py, "z")?.extract(py)?,
                        -normal.getattr(py, "y")?.extract(py)?,
                    ],
                };

                // Update min and max vertex positions
                if vertex.position[0] < vertex_min[0] {
                    vertex_min[0] = vertex.position[0]
                };
                if vertex.position[0] > vertex_max[0] {
                    vertex_max[0] = vertex.position[0]
                };
                if vertex.position[1] < vertex_min[1] {
                    vertex_min[1] = vertex.position[1]
                };
                if vertex.position[1] > vertex_max[1] {
                    vertex_max[1] = vertex.position[1]
                };
                if vertex.position[2] < vertex_min[2] {
                    vertex_min[2] = vertex.position[2]
                };
                if vertex.position[2] > vertex_max[2] {
                    vertex_max[2] = vertex.position[2]
                };

                mesh_vertices.push(vertex);
            }

            // Add vertices to buffer
            vertices_count = mesh_vertices.len();
            mesh_buffer.extend(to_padded_byte_vector(mesh_vertices));
            vertices_length = mesh_buffer.len() - indices_length;
        }

        // Get buffer filename
        let buffer_uri = format!("MESH_{}", mesh_name);

        // Write buffer to disk
        self.write_resource(&buffer_uri, mesh_buffer.as_slice())?;

        // Get the mesh's integer index
        let mesh_index = self.gltf_doc.meshes.len() as u32;

        // Add mesh name to the names index
        self.data_names
            .mesh_names
            .insert(mesh_name.clone(), mesh_index);

        use gltf::validation::Checked::Valid;

        // Create glTF mesh buffer
        let buffer_id = self.gltf_doc.buffers.len();
        self.gltf_doc.buffers.push(gltf::Buffer {
            byte_length: mesh_buffer.len() as u32,
            uri: Some(buffer_uri.clone()),
            extensions: None,
            name: None,
            extras: Default::default(),
        });

        // Create mesh indices buffer view
        let indices_buffer_view_id = self.gltf_doc.buffer_views.len();
        self.gltf_doc.buffer_views.push(gltf::buffer::View {
            buffer: gltf::Index::new(buffer_id as u32),
            byte_length: indices_length as u32,
            byte_offset: None,
            byte_stride: None,
            target: Some(Valid(gltf::buffer::Target::ElementArrayBuffer)),
            extras: None,
            extensions: None,
            name: None,
        });

        // Create mesh vertices buffer view
        let vertices_buffer_view_id = self.gltf_doc.buffer_views.len();
        self.gltf_doc.buffer_views.push(gltf::buffer::View {
            buffer: gltf::Index::new(buffer_id as u32),
            byte_length: vertices_length as u32,
            byte_offset: Some(indices_length as u32),
            byte_stride: Some(gltf::buffer::ByteStride(
                std::mem::size_of::<VertexData>() as u32
            )),
            target: Some(gltf::validation::Checked::Valid(
                gltf::buffer::Target::ArrayBuffer,
            )),
            extras: None,
            extensions: None,
            name: None,
        });

        // Create indices accessor
        let indices_accessor_id = self.gltf_doc.accessors.len();
        self.gltf_doc.accessors.push(gltf::Accessor {
            buffer_view: gltf::Index::new(indices_buffer_view_id as u32),
            byte_offset: 0,
            component_type: Valid(gltf::accessor::GenericComponentType(
                gltf::accessor::ComponentType::U16,
            )),
            count: indices_count as u32,
            type_: Valid(gltf::accessor::Type::Scalar),
            min: None,
            max: None,
            name: None,
            normalized: false,
            sparse: None,
            extras: None,
            extensions: None,
        });

        // Create vertex position accessor
        let vertex_position_accessor_id = self.gltf_doc.accessors.len();
        self.gltf_doc.accessors.push(gltf::Accessor {
            buffer_view: gltf::Index::new(vertices_buffer_view_id as u32),
            byte_offset: 0,
            component_type: Valid(gltf::accessor::GenericComponentType(
                gltf::accessor::ComponentType::F32,
            )),
            count: vertices_count as u32,
            type_: Valid(gltf::accessor::Type::Vec3),
            min: Some(gltf::Value::from(&vertex_min[..])),
            max: Some(gltf::Value::from(&vertex_max[..])),
            name: None,
            normalized: false,
            sparse: None,
            extras: None,
            extensions: None,
        });

        // Create vertex normal accessor
        let vertex_normal_accessor_id = self.gltf_doc.accessors.len();
        self.gltf_doc.accessors.push(gltf::Accessor {
            buffer_view: gltf::Index::new(vertices_buffer_view_id as u32),
            byte_offset: (std::mem::size_of::<f32>() * 3) as u32,
            component_type: Valid(gltf::accessor::GenericComponentType(
                gltf::accessor::ComponentType::F32,
            )),
            count: vertices_count as u32,
            type_: Valid(gltf::accessor::Type::Vec3),
            min: None,
            max: None,
            name: None,
            normalized: false,
            sparse: None,
            extras: None,
            extensions: None,
        });

        // Create mesh primitive
        // TODO: prevent overflows of the mesh indices by splitting up the mesh into
        // multiple primitives.
        let primitive = gltf::mesh::Primitive {
            attributes: {
                let mut map = std::collections::HashMap::new();
                map.insert(
                    Valid(gltf::mesh::Semantic::Positions),
                    gltf::Index::new(vertex_position_accessor_id as u32),
                );
                map.insert(
                    Valid(gltf::mesh::Semantic::Normals),
                    gltf::Index::new(vertex_normal_accessor_id as u32),
                );
                map
            },
            indices: Some(gltf::Index::new(indices_accessor_id as u32)),
            material: Some(gltf::Index::new(0)), // Use a default material for now
            mode: Valid(gltf::mesh::Mode::Triangles),
            targets: None,
            extras: None,
            extensions: None,
        };

        // Add mesh data to the glTF doc
        self.gltf_doc.meshes.push(gltf::Mesh {
            name: Some(mesh_name),
            primitives: vec![primitive],
            weights: None,
            extras: None,
            extensions: None,
        });

        // Return the mesh index
        Ok(mesh_index)
    }
}
