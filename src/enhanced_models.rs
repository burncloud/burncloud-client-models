use dioxus::prelude::*;
use burncloud_service_models::ModelStatus;
use crate::examples::{get_example_installed_models, get_example_available_models};

/// 简化版的增强模型管理 - 仅用于演示 burncloud-service-models 集成
#[component]
pub fn EnhancedModelManagement() -> Element {
    let mut search_term = use_signal(|| String::new());

    // 使用静态数据
    let installed_models = get_example_installed_models();
    let available_models = get_example_available_models();

    rsx! {
        div { class: "model-management-container",
            // 页面头部
            div { class: "page-header",
                div { class: "flex justify-between items-center",
                    div {
                        h1 { class: "text-large-title font-bold text-primary m-0",
                            "增强版模型管理"
                        }
                        p { class: "text-secondary m-0 mt-sm",
                            "使用 burncloud-service-models 的完整功能演示"
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

            // 统计卡片
            div { class: "stats-grid mb-lg",
                StatCard {
                    title: "已安装模型".to_string(),
                    value: installed_models.len().to_string(),
                    icon: "🧠".to_string(),
                    color: "blue".to_string()
                }
                StatCard {
                    title: "运行中".to_string(),
                    value: installed_models.iter().filter(|m| matches!(m.status, ModelStatus::Running)).count().to_string(),
                    icon: "🟢".to_string(),
                    color: "green".to_string()
                }
                StatCard {
                    title: "已停止".to_string(),
                    value: installed_models.iter().filter(|m| matches!(m.status, ModelStatus::Stopped)).count().to_string(),
                    icon: "🔴".to_string(),
                    color: "red".to_string()
                }
                StatCard {
                    title: "可下载".to_string(),
                    value: available_models.len().to_string(),
                    icon: "📥".to_string(),
                    color: "purple".to_string()
                }
            }

            // 模型网格
            div { class: "page-content",
                // 已安装模型部分
                div { class: "mb-xxxl",
                    h2 { class: "text-title font-semibold mb-lg",
                        "已安装模型 ({installed_models.len()})"
                    }
                    div { class: "grid gap-lg", style: "grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));",
                        for model in installed_models.iter() {
                            crate::models::InstalledModelCard { model: model.clone() }
                        }
                    }
                }

                // 可下载模型部分
                div {
                    h2 { class: "text-title font-semibold mb-lg",
                        "可下载模型 ({available_models.len()})"
                    }
                    div { class: "grid gap-lg", style: "grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));",
                        for model in available_models.iter() {
                            crate::models::AvailableModelCard { model: model.clone() }
                        }
                    }
                }

                // 技术说明
                div { class: "mt-xxxl p-lg bg-info-light rounded",
                    h3 { class: "text-subtitle font-semibold mb-md", "🔧 技术实现" }
                    div { class: "grid gap-md", style: "grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));",
                        div { class: "tech-card",
                            h4 { class: "font-semibold mb-sm", "数据源" }
                            ul { class: "text-sm text-secondary",
                                li { "使用 " code { "burncloud-service-models" } " 提供的数据结构" }
                                li { "支持完整的模型元数据" }
                                li { "包含运行时状态管理" }
                                li { "提供系统要求信息" }
                            }
                        }
                        div { class: "tech-card",
                            h4 { class: "font-semibold mb-sm", "功能特性" }
                            ul { class: "text-sm text-secondary",
                                li { "实时状态显示" }
                                li { "智能类型识别" }
                                li { "资源使用统计" }
                                li { "动态评分系统" }
                            }
                        }
                        div { class: "tech-card",
                            h4 { class: "font-semibold mb-sm", "数据结构" }
                            ul { class: "text-sm text-secondary",
                                li { code { "InstalledModel" } " - 已安装模型" }
                                li { code { "AvailableModel" } " - 可下载模型" }
                                li { code { "ModelType" } " - 模型类型枚举" }
                                li { code { "ModelStatus" } " - 运行状态" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(title: String, value: String, icon: String, color: String) -> Element {
    rsx! {
        div { class: "stat-card {color}",
            div { class: "stat-icon", "{icon}" }
            div { class: "stat-content",
                div { class: "stat-value", "{value}" }
                div { class: "stat-title", "{title}" }
            }
        }
    }
}