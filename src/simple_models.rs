use dioxus::prelude::*;
use crate::app_state::AppState;

/// 简化版模型管理组件 - 使用 AppState 获取真实数据
#[component]
pub fn SimpleModelManagement(app_state: AppState) -> Element {
    let mut search_term = use_signal(|| String::new());

    // 从 AppState 获取数据
    let (filtered_installed, filtered_available) = if search_term.read().is_empty() {
        (app_state.installed_models.iter().collect::<Vec<_>>(),
         app_state.available_models.iter().collect::<Vec<_>>())
    } else {
        app_state.search_models(&search_term.read())
    };

    rsx! {
        div { class: "page-header",
            div { class: "flex justify-between items-center",
                div {
                    h1 { class: "text-large-title font-bold text-primary m-0",
                        "模型管理"
                    }
                    p { class: "text-secondary m-0 mt-sm",
                        "管理和部署大语言模型 (使用 burncloud-service-models)"
                    }
                }
                div { class: "flex gap-md",
                    button { class: "btn btn-secondary",
                        span { "🔄" }
                        "刷新"
                    }
                    button { class: "btn btn-secondary",
                        span { "📁" }
                        "浏览本地"
                    }
                    button { class: "btn btn-primary",
                        span { "+" }
                        "添加模型"
                    }
                }
            }
            div { class: "mt-lg",
                input {
                    class: "input",
                    style: "max-width: 400px;",
                    placeholder: "搜索模型...",
                    value: "{search_term}",
                    oninput: move |evt| search_term.set(evt.value())
                }
            }
        }

        div { class: "page-content",
            // 已安装模型
            div { class: "mb-xxxl",
                h2 { class: "text-title font-semibold mb-lg",
                    "已安装模型 ({filtered_installed.len()})"
                }
                if filtered_installed.is_empty() {
                    div { class: "empty-state",
                        p { "没有找到已安装的模型" }
                        if !search_term.read().is_empty() {
                            p { class: "text-secondary", "尝试调整搜索条件" }
                        }
                    }
                } else {
                    div { class: "grid gap-lg",
                        style: "grid-template-columns: 1fr;",
                        for installed_model in filtered_installed {
                            crate::models::InstalledModelCard { model: installed_model.clone() }
                        }
                    }
                }
            }

            // 可安装模型
            div {
                h2 { class: "text-title font-semibold mb-lg",
                    "可安装模型 ({filtered_available.len()})"
                }
                if filtered_available.is_empty() {
                    div { class: "empty-state",
                        p { "没有找到可安装的模型" }
                        if !search_term.read().is_empty() {
                            p { class: "text-secondary", "尝试调整搜索条件" }
                        }
                    }
                } else {
                    div { class: "grid gap-lg",
                        style: "grid-template-columns: 1fr;",
                        for available_model in filtered_available {
                            crate::models::AvailableModelCard { model: available_model.clone() }
                        }
                    }
                }
            }

            // 数据源信息
            div { class: "mt-xxxl p-lg border rounded",
                h3 { class: "text-lg font-semibold mb-md", "📦 数据源信息" }
                div { class: "grid gap-md",
                    style: "grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));",
                    div { class: "stat-item",
                        span { class: "text-secondary", "已安装模型:" }
                        span { class: "font-semibold ml-sm", "{app_state.installed_models.len()}" }
                    }
                    div { class: "stat-item",
                        span { class: "text-secondary", "可用模型:" }
                        span { class: "font-semibold ml-sm", "{app_state.available_models.len()}" }
                    }
                    div { class: "stat-item",
                        span { class: "text-secondary", "数据源:" }
                        span { class: "font-semibold ml-sm",
                            code { "burncloud-service-models" }
                        }
                    }
                }
            }
        }
    }
}