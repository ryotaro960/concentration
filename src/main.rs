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
    use concentration_lib::app::{shell, App};

    let conf = get_configuration(None).unwrap(); // サーバーの IP アドレスやポート番号（どこで待ち構えるか）を取得します。
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // ルートリストを生成
    let routes = generate_route_list(App); // どんなページ（URL）があるのかをリストアップします。

    let app = Router::new() // Axum（Web サーバーエンジン）を初期化し、Leptos の画面を表示するための設定を紐付けます。
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone()) // HTML の土台（<head> や <body> タグなど）を定義している場所を指しています。
        })
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap(); // サーバーを指定のポートで待ち受け状態にする際、準備ができるまで待機します。
                                                                        // TcpListener::bind(&addr) で指定したアドレス（例: 127.0.0.1:3000）の門を開きます。
    axum::serve(listener, app).await.unwrap(); // Web サーバーを起動し、リクエストが来るのをずっと待ち続けます。
    // 最後に serve を実行することで、プログラムは終了せずにずっと動き続け、ユーザーがブラウザでアクセスしてくるのを待ち構える状態になります。
}

#[cfg(not(feature = "ssr"))]

fn main() {
    // UIのルートとなるコンポーネント（App）をインポートしています。
    // ポイント: Rustのライブラリ（lib.rs）側にメインのロジックやコンポーネントを配置し、それをバイナリ側から呼び出す構成になっています。これにより、Hydration（サーバーとクライアントの同期）がスムーズに行えます。
    // crate ではなく concentration (プロジェクト名) から呼び出す
    use concentration_lib::app::App;

    // Rustコードがパニック（実行時エラー）を起こした際、ブラウザのデバッグコンソールに読みやすいエラーメッセージを表示させるための設定です。
    // 重要性: これがないと、ブラウザ上では「Runtime Error」としか表示されず、原因の特定が非常に困難になります。
    console_error_panic_hook::set_once();

    // Appコンポーネントを、HTMLの<body>タグ内にレンダリング（マウント）します。
    // 動作: Leptosはこの時点で、DOMの制御権をRust（WebAssembly）側に引き渡します。
    mount_to_body(App);
}