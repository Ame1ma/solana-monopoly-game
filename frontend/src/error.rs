use dioxus::prelude::*;

use super::ERROR;

#[component]
pub fn ErrorPage(err: String) -> Element {
    rsx! {
        div { class: "flex flex-col h-screen items-center justify-center",
            p { class: "my-3 w-9/12 text-center font-mono text-4xl font-bold", "发生错误" }
            p { class: "my-3 w-9/12 text-center font-mono text-2xl font-bold", "{err}" }
            button {
                class: "my-3 inline-flex items-center justify-center w-9/12 px-8 py-3 text-lg font-bold text-white transition-all duration-200 bg-gray-900 border-2 border-transparent md:w-5/12 rounded-xl hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-gray-900",
                onclick: move |_| {
                    *ERROR.write() = None;
                },
                "返回"
            }
        }
    }
}
