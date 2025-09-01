use godot::classes::{
    BoxMesh, INode3D, Material, Mesh, MeshInstance3D, Node, Node3D, SphereMesh, StandardMaterial3D,
};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct WorldBuilder {
    #[base]
    base: Base<Node3D>,
}

#[godot_api]
impl WorldBuilder {
    #[func]
    pub fn create_reference_objects(&mut self) {
        // キューブをいくつか配置
        let cube_positions = [
            Vector3::new(10.0, 0.0, 0.0),
            Vector3::new(-10.0, 0.0, 0.0),
            Vector3::new(0.0, 0.0, 10.0),
            Vector3::new(0.0, 0.0, -10.0),
            Vector3::new(15.0, 5.0, 15.0),
            Vector3::new(-15.0, 3.0, -15.0),
        ];

        for (i, position) in cube_positions.iter().enumerate() {
            let mut mesh_instance = MeshInstance3D::new_alloc();
            let mut box_mesh = BoxMesh::new_gd();
            box_mesh.set_size(Vector3::new(2.0, 2.0, 2.0));
            mesh_instance.set_mesh(&box_mesh.upcast::<Mesh>());

            // マテリアルを設定（色を変える）
            let mut material = StandardMaterial3D::new_gd();
            let color = match i % 3 {
                0 => Color::RED,
                1 => Color::BLUE,
                _ => Color::GREEN,
            };
            material.set_albedo(color);
            mesh_instance.set_material_override(&material.upcast::<Material>());

            mesh_instance.set_position(*position);
            self.base_mut().add_child(&mesh_instance.upcast::<Node>());
        }

        // スフィアをいくつか配置
        let sphere_positions = [
            Vector3::new(5.0, 8.0, 5.0),
            Vector3::new(-5.0, 6.0, -5.0),
            Vector3::new(20.0, 2.0, 0.0),
            Vector3::new(0.0, 10.0, 20.0),
        ];

        for (i, position) in sphere_positions.iter().enumerate() {
            let mut mesh_instance = MeshInstance3D::new_alloc();
            let mut sphere_mesh = SphereMesh::new_gd();
            sphere_mesh.set_radius(1.5);
            sphere_mesh.set_height(3.0);
            mesh_instance.set_mesh(&sphere_mesh.upcast::<Mesh>());

            // マテリアルを設定
            let mut material = StandardMaterial3D::new_gd();
            let color = match i % 2 {
                0 => Color::YELLOW,
                _ => Color::MAGENTA,
            };
            material.set_albedo(color);
            mesh_instance.set_material_override(&material.upcast::<Material>());

            mesh_instance.set_position(*position);
            self.base_mut().add_child(&mesh_instance.upcast::<Node>());
        }

        godot_print!("Reference objects created by WorldBuilder");
    }

    #[func]
    pub fn create_ground(&mut self) {
        // 地面として大きな平面を作成
        let mut ground = MeshInstance3D::new_alloc();
        let mut ground_mesh = BoxMesh::new_gd();
        ground_mesh.set_size(Vector3::new(100.0, 0.5, 100.0));
        ground.set_mesh(&ground_mesh.upcast::<Mesh>());

        let mut ground_material = StandardMaterial3D::new_gd();
        ground_material.set_albedo(Color::from_rgb(0.5, 0.5, 0.5)); // グレー
        ground.set_material_override(&ground_material.upcast::<Material>());

        ground.set_position(Vector3::new(0.0, -2.0, 0.0));
        self.base_mut().add_child(&ground.upcast::<Node>());

        godot_print!("Ground created by WorldBuilder");
    }

    #[func]
    pub fn create_world(&mut self) {
        self.create_ground();
        self.create_reference_objects();
    }
}

#[godot_api]
impl INode3D for WorldBuilder {
    fn init(base: Base<Node3D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        // デフォルトでワールドを作成
        self.create_world();
    }
}
