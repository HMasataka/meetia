use godot::classes::{INode3D, Node3D, XrController3D, XrInterface, XrServer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct VRPlayer {
    speed: f32,
    smooth_turn_speed: f32,
    snap_turn_angle: f32,
    teleport_distance: f32,
    xr_interface: Option<Gd<XrInterface>>,
    base: Base<Node3D>,
}

#[godot_api]
impl VRPlayer {
    #[func]
    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
    }

    #[func]
    pub fn set_smooth_turn_speed(&mut self, turn_speed: f32) {
        self.smooth_turn_speed = turn_speed;
    }

    #[func]
    pub fn set_snap_turn_angle(&mut self, angle: f32) {
        self.snap_turn_angle = angle;
    }

    #[func]
    pub fn set_teleport_distance(&mut self, distance: f32) {
        self.teleport_distance = distance;
    }

    fn initialize_vr(&mut self) -> bool {
        let mut xr_server = XrServer::singleton();

        // Find the OpenXR interface
        let interface_name = GString::from("OpenXR");
        if let Some(mut interface) = xr_server.find_interface(&interface_name) {
            self.xr_interface = Some(interface.clone());

            // Initialize the interface
            if interface.initialize() {
                // Set the primary interface
                xr_server.set_primary_interface(Some(&interface));
                godot_print!("VR interface initialized successfully");
                return true;
            } else {
                godot_print!("Failed to initialize VR interface");
            }
        } else {
            godot_print!("OpenXR interface not found");
        }

        false
    }

    fn get_xr_origin(&self) -> Option<Gd<Node3D>> {
        // Find XROrigin3D in the scene
        let scene_root = self.base().get_tree()?.get_current_scene()?;
        scene_root
            .find_child("XROrigin3D")
            .map(|node| node.cast::<Node3D>())
    }

    fn get_controller_input(&self, controller_name: &str) -> Vector2 {
        if let Some(origin) = self.get_xr_origin() {
            if let Some(controller_node) = origin.find_child(controller_name) {
                if let Ok(controller) = controller_node.try_cast::<XrController3D>() {
                    // Get thumbstick input from the controller
                    return controller.get_vector2("move");
                }
            }
        }
        Vector2::ZERO
    }

    fn is_button_pressed(&self, controller_name: &str, button_name: &str) -> bool {
        if let Some(origin) = self.get_xr_origin() {
            if let Some(controller_node) = origin.find_child(controller_name) {
                if let Ok(controller) = controller_node.try_cast::<XrController3D>() {
                    return controller.is_button_pressed(button_name);
                }
            }
        }
        false
    }

    fn handle_movement(&mut self, delta: f64) {
        let left_stick = self.get_controller_input("LeftHand");
        let right_stick = self.get_controller_input("RightHand");

        if let Some(mut origin) = self.get_xr_origin() {
            // Movement with left stick
            if left_stick.length() > 0.1 {
                let movement_speed = self.speed * delta as f32 * left_stick.length();

                // Get the camera's forward and right vectors
                if let Some(camera_node) = origin.find_child("XRCamera3D") {
                    if let Ok(camera) = camera_node.try_cast::<Node3D>() {
                        let transform = camera.get_global_transform();
                        let forward = -transform.basis.col_c(); // Forward is -Z
                        let right = transform.basis.col_a(); // Right is X

                        // Project forward vector onto horizontal plane
                        let mut horizontal_forward = forward;
                        horizontal_forward.y = 0.0;
                        horizontal_forward = horizontal_forward.normalized();

                        let mut horizontal_right = right;
                        horizontal_right.y = 0.0;
                        horizontal_right = horizontal_right.normalized();

                        // Calculate movement vector
                        let movement =
                            horizontal_forward * left_stick.y + horizontal_right * left_stick.x;
                        let movement_vector = movement * movement_speed;

                        // Apply movement to XR origin
                        let current_pos = origin.get_position();
                        origin.set_position(current_pos + movement_vector);
                    }
                }
            }

            // Rotation with right stick
            if right_stick.x.abs() > 0.1 {
                let rotation_speed = self.smooth_turn_speed * delta as f32;
                let rotation_amount = -right_stick.x * rotation_speed;
                origin.rotate_y(rotation_amount);
            }
        }
    }

    fn handle_teleportation(&mut self) {
        // Check for teleport button (trigger on right controller)
        if self.is_button_pressed("RightHand", "select") {
            if let Some(mut origin) = self.get_xr_origin() {
                if let Some(right_hand_node) = origin.find_child("RightHand") {
                    if let Ok(right_hand) = right_hand_node.try_cast::<XrController3D>() {
                        // Cast a ray forward from the controller
                        let transform = right_hand.get_global_transform();
                        let ray_origin = transform.origin;
                        let ray_direction = -transform.basis.col_c(); // Forward direction

                        // Simple teleportation - move forward by teleport_distance
                        let teleport_target = ray_origin + ray_direction * self.teleport_distance;

                        // Project to ground level (y = 0)
                        let mut target_position = origin.get_position();
                        target_position.x = teleport_target.x;
                        target_position.z = teleport_target.z;

                        origin.set_position(target_position);

                        godot_print!("Teleported to: {:?}", target_position);
                    }
                }
            }
        }
    }
}

#[godot_api]
impl INode3D for VRPlayer {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            speed: 3.0,
            smooth_turn_speed: 2.0,
            snap_turn_angle: 30.0,
            teleport_distance: 5.0,
            xr_interface: None,
            base,
        }
    }

    fn ready(&mut self) {
        // Initialize VR
        if !self.initialize_vr() {
            godot_print!("VR initialization failed, falling back to desktop mode");
        }

        godot_print!("VR Player initialized");
    }

    fn physics_process(&mut self, delta: f64) {
        // Only process VR input if VR is active
        if let Some(ref interface) = self.xr_interface {
            if interface.is_initialized() {
                self.handle_movement(delta);
                self.handle_teleportation();
            }
        }
    }
}
