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
    // view! マクロは、「Rustのコードの中でHTML（宣言的UI）を記述するための専用ツール」という意味
    // view! の中では、HTMLタグ（<tag>）や属性（attr="value"）を直接書くことができます。
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

// #[component]: 関数をLeptosのコンポーネントとして定義するためのマクロです。
// これにより、HTMLタグのように <App/> として呼び出せるようになります。
#[component]
// -> impl IntoView:「Viewに変換可能なものを返す」という宣言です。LeptosのUI要素を返す関数の標準的な戻り値の型です。
pub fn App() -> impl IntoView {
    provide_meta_context(); // HTMLの <head> 内にある情報（タイトル、メタタグ、スタイルシートなど）を、コンポーネントツリーのどこからでも書き換えられるようにするための準備です。

    view! {
        <Stylesheet id="leptos" href="/pkg/concentration.css"/> // CSSファイルを読み込みます。href="/pkg/concentration.css" は、ビルド時に生成されるスタイルシートを指しています。
        <Title text="神経衰弱ゲーム"/> // ブラウザのタブに表示されるタイトルを設定します。

        <Router> // ルーティング機能のコンテキストを提供します。アプリ全体をこれで包むのが一般的です。
            <main>
                 // 複数のルート定義をまとめます。
                <Routes fallback=|| "Page not found.".into_view()> // 定義されていないURL（404エラー）にアクセスした際に表示する内容を指定します。ここでは "Page not found." というテキストを表示するように設定されています。
                    <Route path=StaticSegment("") // ルートディレクトリ（トップページ /）を指します。
                    view=HomePage/>  // そのURLにアクセスしたときに HomePage という別のコンポーネントを表示することを意味します。
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
    // 構造体のフィールドのようなもの。ただし、ただの変数ではなく「この値が変わったら、関係するUIに通知せよ」という特殊な能力を持った「シグナル」です。
    // set_is_flipped を使って値を書き換えると、Leptosが「あ！値が変わった！この値を使っているHTMLの部分だけ書き換えなきゃ！」と自動で動いてくれます。
    let (is_flipped, set_is_flipped) = signal(false);

    // これはオブジェクトのメソッドに相当します。カードがクリックされた時の動作（状態の反転）を定義しています。
    let onclick = move |_| {
        // update メソッドの動きを言葉にすると、以下のようになります。
        // set_is_flipped が、現在のシグナルの中身（false など）を取り出す。
        // その値を、関数の引数（flipped）として渡す。
        // その flipped を使って新しい値を計算し、書き換える。
        set_is_flipped.update(|flipped| *flipped = !*flipped);// この中の flipped は、「今現在のシグナルの値」を一時的に代入して渡されたものです。
    };

    view! {
        <div class="card-container" on:click=onclick>
            // 「もし is_flipped が true なら is-flipped というCSSクラスを付与せよ」という動的な紐付けを行っています。
            // move || という記述は、Rustの「クロージャ（名前のない使い捨て関数）」を作るための構文です。
            // クロージャは |引数| { 処理 } と書きます。 ||: 引数が「なし」であることを意味します。
            // move || is_flipped.get(): 「何も受け取らずに、is_flipped.get() の値を返す関数」をその場で作っています。
            // 関数（クロージャ）にして渡すことで、Leptosが「後で値が変わった時に、この関数をもう一度実行して、新しい値を確認する」ことができるようになります。
            <div class="card-inner" class:is-flipped=move || is_flipped.get()>
                <div class="card-face card-front">"?"</div>
                <div class="card-face card-back">{content}</div>
            </div>
        </div>
    }
}