// 全体レイアウト、ゲームの状態管理（メソッド）

use leptos::prelude::*; // signal, IntoView, など基本機能をすべてインポート
use leptos::component; // #[component] マクロ
use leptos::view;      // view! マクロ
use leptos_meta::{provide_meta_context, Stylesheet, Title, MetaTags};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;
use rand::seq::SliceRandom; // シャッフルに必要
use rand::thread_rng;
use std::time::Duration;

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
                <App/> // <body> タグの中で <App/> を呼び出すことで、作成したゲーム本体を画面に流し込みます。
            </body>
        </html>
    }
}

// mainから最初に呼び出す関数(webページの構成)
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

// メインの処理を行う
#[component]
fn HomePage() -> impl IntoView {
    // 1. カードのデータを用意
    /*
    let mut cards_data = vec![
        "1", "1", "2", "2", "3", "3", "4", "4", "5", "5",
        "6", "6", "7", "7", "8", "8", "9", "9", "10", "10"
    ];
    */

    let mut cards_data = vec![
        ["1","♠"], ["1","♦"], ["2","♠"], ["2","♦"], 
        ["3","♠"], ["3","♦"], ["4","♠"], ["4","♦"],
        ["5","♠"], ["5","♦"], ["6","♠"], ["6","♦"],
        ["7","♠"], ["7","♦"], ["8","♠"], ["8","♦"],
        ["9","♠"], ["9","♦"], ["10","♠"], ["10","♦"]
    ];

    // 2. 乱数生成器を使ってシャッフル
    let mut rng = thread_rng(); // 現在のスレッドで使用する乱数生成器を取得します。
    cards_data.shuffle(&mut rng);

    // 3. めくられたカードの「インデックス」を管理するシグナル 「ゲームの現在の進行状況」をリアルタイムに管理するための箱
    // flipped_indices（読み取り専用）
    // 現在の状態（どのカードがめくられているか）を取得するための変数です。
    // 中身は Vec<usize>、つまり「0, 1, 5」といった数値のリストです(カードのインデックス番号のリスト)。

    // set_flipped_indices（書き込み専用）
    // 状態を更新するための関数（セッター）です。
    // カードをクリックしたときなどに、この関数を使ってリストに新しい番号を追加したり、リストを空にしたりします。

    // signal(Vec::<usize>::new())
    // 初期値として「空のリスト（何もめくられていない状態）」をセットして、シグナルを作成しています。
    let (flipped_indices, set_flipped_indices) = signal(Vec::<usize>::new()); // 変数「flipped」の定義
    // なぜ「Signal」を使うのか？
    // 自動更新: set_flipped_indices を使ってリストの中身が変わると、その値を使っている画面上のパーツ（カードの絵柄など）が自動的に再描画されます。
    // 効率的: 画面全体を書き換えるのではなく、値が変わった「その場所だけ」をピンポイントで更新するため、非常に高速に動作します。

    // 4. クリック時の処理
    // idx（カードのインデックス番号）を引数に取るクロージャ（関数）を定義しています。
    // move キーワードは、クロージャの外側にある変数（この場合は set_flipped_indices）の所有権をクロージャ内に取り込むことを意味します。
    let select_card = move |idx: usize| {
        // set_flipped_indices は、現在「表を向いているカードの番号」を保持している状態（シグナル）を更新するための関数です。
        // .update() を使うことで、現在の値（indices）を直接書き換えることができます。
        set_flipped_indices.update(|indices| {

            // !indices.contains(&idx): すでに選択済みの（表を向いている）同じカードを二度押ししても反応しないようにします。
            if indices.len() >= 2 || indices.contains(&idx) {
                return;
            }

            // 1枚目または2枚目として追加
            indices.push(idx);

            // 2枚になったら、1秒後にクリアする予約を入れる
            if indices.len() >= 2 {
                // 非同期で1秒後に空にする
                set_timeout(move || {
                    // indicesを空にする
                    set_flipped_indices.set(vec![]);
                }, Duration::from_secs(1));
            }
        });
    };

    // ブラウザで表示する内容
    view! {
        <h1 style="text-align: center;">"Card Click Demo"</h1>
        
        // カードを1行5枚、100px間隔で並べる
        <div style="
            display: grid; 
            grid-template-columns: repeat(5, 100px);  
            gap: 20px; 
            justify-content: center; 
            margin-top: 20px;
        ">
            // .enumerate().map(|(idx, content)| ...) とすることで、各カードに 0 から 19 までの背番号（idx）を振っています。
            // .into_iter(): 配列（ベクタ）の要素を一つずつ取り出せるようにします。
            // .enumerate(): 要素そのもの (content) だけでなく、「何番目のカードか」という番号 (idx) をセットで取得します。データが流れる瞬間に、横から「0番、1番…」と番号を振ります。
            // .map(|(idx, content)| ... ): 取り出したデータを、view!（HTML要素）へ作り変える処理です。
            {cards_data.into_iter().enumerate().map(|(idx, content /* このカッコの中が「変数の定義」です */)| {
                view! {
                    <Card 
                        content=content
                        // 「このカードが開いているかどうか」の判定式をクロージャとして渡しています。
                        // 「現在開いている番号リスト (flipped_indices) の中に、自分の番号 (idx) が含まれているか？」を常にチェックしています。
                        // リストが更新されると、この判定が自動で再計算され、カードがパタパタと裏返ります。
                        is_open=move || flipped_indices.get().contains(&idx)
                        // クリックされた時に実行する関数です。自分の番号 (idx) を引数として select_card 関数に伝えます。
                        on_click=move |_| select_card(idx)
                    />
                }
            }).collect_view()} // map でバラバラに生成された複数の Card を、Leptos が画面に描画できる一つの形式にまとめ上げるメソッドです。
        </div>
    }
}

// 関数シグネチャの基本形、標準的なルールです。
/*
fn コンポーネント名(プロパティ(受け取るデータのこと)) -> impl IntoView {
    view! {
        // ここにHTML風のコード
    }
}
*/
// この Card 関数は、親から3つのデータを受け取ります。
// content: カードの裏面に表示する文字（例: "A" や "りんご" など）。
// is_open: 「今、このカードは表を向いているべきか？」を判定する関数（クロージャ）。
// on_click: カードがクリックされたときに実行される処理（親側で定義された選択ロジックなど）。
#[component]
fn Card(
    content: [&'static str; 2], // 要素数2のstr配列として受け取る
    // 関数（クロージャ）をPropsとして渡すときは、以下の3つをセットで書くと覚えておくとエラーに悩まされにくくなります。
    // impl Fn(...): 実行可能な関数であること。
    // + Send: スレッド間を移動できること。
    // + 'static: アプリが動いている間、ずっと有効であること。
    is_open: impl Fn() -> bool + Send + 'static, // 親の状態を読み取る関数
    on_click: impl Fn(leptos::ev::MouseEvent) + Send + 'static // 親に通知する関数

    // impl Trait という書き方は、「このトレイト（インターフェース）を実装している何か」を返すという意味です。
    // IntoView トレイトとは、Leptosにおいて「ブラウザに表示可能なもの」を表すトレイトです。
    // view! { ... } マクロが生成する値は、最終的にこの IntoView を実装した型になります。
) -> impl IntoView {

    view! {
        <div class="card-container" on:click=on_click>
            // 親から渡された is_open() の結果でクラスを切り替える
            <div class="card-inner" class:is-flipped=move || is_open()>
                <div class="card-face card-front">"?"</div>
                <div class="card-face card-back">
                    // 左上のマーク
                    <div class="corner top-left">{content[1]}</div>
                    
                    // 中央のメインコンテンツ
                    <div class="main-content">{content[0]}</div>
                    
                    // 右下のマーク
                    <div class="corner bottom-right">{content[1]}</div>
                </div>
            </div>
        </div>
    }
}