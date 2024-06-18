use dioxus::prelude::*;

use super::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col h-screen items-center justify-center",
            h1 { class: "mb-60 text-center font-mono text-6xl font-bold", "=大富翁=" }
            button {
                class: "md:w-5/12 my-3 inline-flex items-center justify-center w-9/12 px-8 py-3 text-lg font-bold text-white transition-all duration-200 bg-gray-900 border-2 border-transparent rounded-xl hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900",
                onclick: move |_| {
                    navigator().push(Route::Create {});
                },
                "创建房间"
            }
            button {
                class: "md:w-5/12 my-3 inline-flex items-center justify-center w-9/12 px-8 py-3 text-lg font-bold text-gray-900 hover:text-white transition-all duration-200 bg-gray-100 border-2 border-gray-900 rounded-xl hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900",
                onclick: move |_| {
                    navigator().push(Route::Join {});
                },
                "加入房间"
            }
        }
    }
}
