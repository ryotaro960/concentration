// 全体レイアウト、ゲームの状態管理（メソッド）

use leptos::prelude::*; // signal, IntoView, など基本機能をすべてインポート
use leptos::component; // #[component] マクロ
use leptos::view;      // view! マクロ
use leptos_meta::{provide_meta_context, Stylesheet, Title, MetaTags};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

// 「HTML全体の骨組み（外枠）を定義する特別な関数」
pub fn shell(options: LeptosOptions) -> impl IntoView {
    // HTMLドキュメントの構造定義
    view! {
        <!DOCTYPE html>
        <html lang="en">
            // Leptos が提供する特殊なコンポーネントを <head> 内に配置し、アプリを正常に動かすための準備をします。
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() /> // 開発中にコードを書き換えた際、ブラウザを自動更新する機能を有効にします。
                <HydrationScripts options/> // サーバーから送られてきた静的なHTMLを、ブラウザ上で動的なアプリ（Rust/WASM）として「起動（Hydrate）」させるための JavaScript を読み込みます。
                <MetaTags/> // App コンポーネント内で指定した <Title> やメタデータを、この場所に反映させます。
            </head>
            <body>
                // <body> タグの中で <App/> を呼び出すことで、作成したゲーム本体を画面に流し込みます。
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/concentration.css"/>
        <Title text="神経衰弱ゲーム"/>

        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    // 1. カードのデータを用意
    let cards = vec!["A", "A", "B", "B", "C", "C", "D", "D"];

    view! {
        <h1>"Card Click Demo"</h1>
        // 2. グリッドレイアウトでカードを配置
        <div style="
            display: grid; 
            grid-template-columns: repeat(4, 100px); 
            gap: 10px; 
            justify-content: center; 
            margin-top: 20px;
        ">
            {cards.into_iter()
                .map(|content| view! { <Card content=content /> })
                .collect_view()}
        </div>
    }
}

#[component]
fn Card(content: &'static str) -> impl IntoView {
    let (is_flipped, set_is_flipped) = signal(false);

    let onclick = move |_| {
        set_is_flipped.update(|flipped| *flipped = !*flipped);
    };

    view! {
        <div class="card-container" on:click=onclick>
            <div class="card-inner" class:is-flipped=move || is_flipped.get()>
                <div class="card-face card-front">"?"</div>
                <div class="card-face card-back">{content}</div>
            </div>
        </div>
    }
}