use godot::classes::{GltfDocument, GltfState, HttpRequest, INode3D, Node, Node3D, XrServer};
use godot::global::Error;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct OnlineGltfLoader {
    #[base]
    base: Base<Node3D>,
    url: GString,
}

#[godot_api]
impl OnlineGltfLoader {
    #[func]
    pub fn set_url(&mut self, url: GString) {
        self.url = url;
    }

    // モデルの方向を修正するためのヘルパーメソッド
    fn correct_model_orientation(node: &mut Gd<Node3D>) {
        // glTFモデルは多くの場合、Y軸が上向きで設計されているが、
        // Godotでは異なる座標系を使用する場合があります。
        // 一般的な修正:
        // 1. Y軸周りに180度回転（前後を反転）
        // 2. X軸周りに-90度回転（Y-up を Z-up に変換）

        // モデルを正しい向きに調整
        node.rotate_y(std::f32::consts::PI / 2.0); // 90度回転

        // 必要に応じてスケール調整
        let current_scale = node.get_scale();
        if current_scale.length() < 0.1 {
            // モデルが小さすぎる場合はスケールアップ
            node.set_scale(Vector3::new(10.0, 10.0, 10.0));
        } else if current_scale.length() > 50.0 {
            // モデルが大きすぎる場合はスケールダウン
            node.set_scale(Vector3::new(0.1, 0.1, 0.1));
        }

        godot_print!(
            "Model orientation corrected: rotation_y=90°, scale={}",
            node.get_scale()
        );
    }

    // HttpRequest のシグナルを受けるメソッド
    #[func]
    fn _on_request_completed(
        &mut self,
        result: i64,
        response_code: i64,
        _headers: PackedStringArray,
        body: PackedByteArray,
    ) {
        if result != 0 || response_code != 200 {
            godot_error!("HTTP failed: result={result} code={response_code}");
            return;
        }

        // glTF をメモリから取り込み
        let mut doc = GltfDocument::new_gd();
        let mut state = GltfState::new_gd();

        // 外部依存解決のため base_path をセット（.glb でも設定しておくと安全）
        state.set_base_path("user://remote_assets/");
        // フォルダは GDScript 側や起動時に作っておくとよい

        let err = doc.append_from_buffer(&body, &state.get_base_path(), &state);
        if err != Error::OK {
            godot_error!("GLTF append_from_buffer failed: {:?}", err);
            return;
        }

        // シーン化してツリーに追加
        if let Some(scene_root) = doc.generate_scene(&state) {
            // モデルの方向を修正（多くのglTFモデルは初期状態で回転が必要）
            let mut corrected_scene =
                if let Ok(mut node3d) = scene_root.clone().try_cast::<Node3D>() {
                    Self::correct_model_orientation(&mut node3d);
                    node3d.upcast::<Node>()
                } else {
                    scene_root
                };

            // VRモードかデスクトップモードかを検出して適切なプレイヤーを選択
            let xr_server = XrServer::singleton();
            let is_vr_active = if let Some(primary_interface) = xr_server.get_primary_interface() {
                primary_interface.is_initialized()
            } else {
                false
            };

            if is_vr_active {
                // VRモードの場合、VRプレイヤーはすでにシーンに存在するはず
                // GLTFモデルを適切な位置に配置
                if let Ok(mut node3d_scene) = corrected_scene.clone().try_cast::<Node3D>() {
                    node3d_scene.set_position(Vector3::new(0.0, 0.0, -2.0)); // プレイヤーの前方に配置
                    corrected_scene = node3d_scene.upcast::<Node>();
                }
                self.base_mut().add_child(&corrected_scene.upcast::<Node>());
                godot_print!("GLTF model loaded in VR mode");
            } else {
                // デスクトップモードの場合、従来のPlayer3Dを使用
                let mut player_controller = crate::player::Player3D::new_alloc();
                player_controller.bind_mut().set_speed(3.0);
                player_controller.bind_mut().set_rotation_speed(1.5);

                // GLTFモデルをプレイヤーコントローラーの子として追加
                player_controller.add_child(&corrected_scene.upcast::<Node>());

                // プレイヤーコントローラーをシーンに追加
                self.base_mut()
                    .add_child(&player_controller.upcast::<Node>());

                godot_print!("GLTF model loaded in desktop mode");
            }
        }
    }
}

#[godot_api]
impl INode3D for OnlineGltfLoader {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
            url: GString::new(),
        }
    }

    fn ready(&mut self) {
        // HttpRequest ノードを用意
        let mut http = HttpRequest::new_alloc();

        // シグナル接続（request_completed(result, response_code, headers, body)）
        // ※ API は crate 版により若干記法が異なります。docs.rs の
        // SignalsOfHttpRequest::request_completed を参照してください。
        let callable = Callable::from_object_method(&self.to_gd(), "_on_request_completed");
        http.connect("request_completed", &callable);

        // ツリーに追加してリクエスト
        self.base_mut().add_child(&http.clone().upcast::<Node>());
        let url = if self.url.is_empty() {
            // Khronos Group の公式サンプル glTF ファイル
            "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/Duck/glTF-Binary/Duck.glb".into()
        } else {
            self.url.clone()
        };
        let err = http.request(&url);
        if err != Error::OK {
            godot_error!("HttpRequest.request error: {:?}", err);
        }
    }
}
