use godot::classes::{GltfDocument, GltfState, HttpRequest, INode3D, Node, Node3D};
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
            self.base_mut().add_child(&scene_root.upcast::<Node>());
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

