// アプリのエントリーポイント
// 0.7系では prelude をインポートするのが最も確実です
// Leptosで「クリックして状態を変える」仕組みは、Signal（シグナル） という機能を使います。


use leptos::prelude::*;
use concentration_lib::app::{App, shell};

// 効率的に並列処理を行うために async（非同期）が必要になります。
#[cfg(feature = "ssr")]
#[tokio::main] // ← これが魔法の呪文（ランタイムの起動）
async fn main() { // 「非同期（Asynchronous）処理をプログラムの開始地点（エントリポイント）で実行できるようにするもの」です。
    use axum::Router;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    // コンパイラの提案（concentration_lib）を試します。
    // もしこれでもエラーが出るなら「concentration」に戻してください。
    use concentration_lib::app::{shell, App};
    // use concentration::app::{shell, App}; // プロジェクト名が concentration の場合

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // ルートリストを生成
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap(); // サーバーを指定のポートで待ち受け状態にする際、準備ができるまで待機します。
    axum::serve(listener, app).await.unwrap(); // Web サーバーを起動し、リクエストが来るのをずっと待ち続けます。
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // crate ではなく concentration (プロジェクト名) から呼び出す
    use concentration_lib::app::App;

    console_error_panic_hook::set_once();
    mount_to_body(App);
}