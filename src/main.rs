use dioxus::prelude::*;
use burncloud_client_models::{
    ModelManagement, SimpleModelManagement, EnhancedModelManagement, ModelStats
};

fn main() {
    LaunchBuilder::desktop()
        .with_cfg(desktop! {
            dioxus_desktop::Config::new()
                .with_window(
                    dioxus_desktop::WindowBuilder::new()
                        .with_title("BurnCloud 模型管理 - burncloud-service-models 集成演示")
                        .with_inner_size(dioxus_desktop::LogicalSize::new(1200.0, 800.0))
                )
        })
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut current_view = use_signal(|| "original".to_string());

    rsx! {
        style { {include_str!("../assets/styles.css")} }

        div { class: "app",
            // 导航栏
            nav { class: "nav",
                div { class: "nav-container",
                    div { class: "nav-brand",
                        h1 { class: "text-lg font-bold", "🔥 BurnCloud 模型管理" }
                        p { class: "text-sm text-secondary", "burncloud-service-models 集成演示" }
                    }
                    div { class: "nav-tabs",
                        button {
                            class: if current_view.read().as_str() == "original" { "nav-tab active" } else { "nav-tab" },
                            onclick: move |_| current_view.set("original".to_string()),
                            "🏠 原版界面"
                        }
                        button {
                            class: if current_view.read().as_str() == "simple" { "nav-tab active" } else { "nav-tab" },
                            onclick: move |_| current_view.set("simple".to_string()),
                            "✨ 简化版"
                        }
                        button {
                            class: if current_view.read().as_str() == "enhanced" { "nav-tab active" } else { "nav-tab" },
                            onclick: move |_| current_view.set("enhanced".to_string()),
                            "🚀 增强版"
                        }
                        button {
                            class: if current_view.read().as_str() == "stats" { "nav-tab active" } else { "nav-tab" },
                            onclick: move |_| current_view.set("stats".to_string()),
                            "📊 统计"
                        }
                    }
                }
            }

            // 主内容区域
            main { class: "main-content",
                match current_view.read().as_str() {
                    "original" => rsx! {
                        div { class: "view-container",
                            div { class: "view-header",
                                h2 { class: "text-xl font-bold", "🏠 原版模型管理界面" }
                                p { class: "text-secondary", "基于硬编码数据的传统界面" }
                            }
                            ModelManagement {}
                        }
                    },
                    "simple" => rsx! {
                        div { class: "view-container",
                            div { class: "view-header",
                                h2 { class: "text-xl font-bold", "✨ 简化版模型管理" }
                                p { class: "text-secondary", "使用 burncloud-service-models 数据源，支持搜索过滤" }
                            }
                            SimpleModelManagement {}
                        }
                    },
                    "enhanced" => rsx! {
                        div { class: "view-container",
                            div { class: "view-header",
                                h2 { class: "text-xl font-bold", "🚀 增强版模型管理" }
                                p { class: "text-secondary", "完整的模型管理界面，展示所有 burncloud-service-models 功能" }
                            }
                            EnhancedModelManagement {}
                        }
                    },
                    "stats" => rsx! {
                        div { class: "view-container",
                            div { class: "view-header",
                                h2 { class: "text-xl font-bold", "📊 模型统计分析" }
                                p { class: "text-secondary", "基于 burncloud-service-models 数据的统计图表" }
                            }
                            ModelStats {}
                        }
                    },
                    _ => rsx! { div { "未知页面" } }
                }
            }

            // 底部信息
            footer { class: "footer",
                div { class: "footer-content",
                    div { class: "footer-section",
                        h4 { class: "font-semibold mb-2", "🔧 技术栈" }
                        ul { class: "text-sm text-secondary",
                            li { "🦀 Rust + Dioxus 前端框架" }
                            li { "📦 burncloud-service-models 数据层" }
                            li { "🎨 现代化 CSS 样式" }
                            li { "⚡ 响应式设计" }
                        }
                    }
                    div { class: "footer-section",
                        h4 { class: "font-semibold mb-2", "📊 数据对比" }
                        ul { class: "text-sm text-secondary",
                            li { "原版: 硬编码的静态数据" }
                            li { "简化版: service-models 示例数据" }
                            li { "增强版: 完整的模型管理功能" }
                            li { "统计: 实时数据分析" }
                        }
                    }
                    div { class: "footer-section",
                        h4 { class: "font-semibold mb-2", "🚀 新特性" }
                        ul { class: "text-sm text-secondary",
                            li { "✅ 统一的数据模型" }
                            li { "✅ 类型安全的 API" }
                            li { "✅ 动态状态管理" }
                            li { "✅ 可扩展架构" }
                        }
                    }
                }
            }
        }
    }
}