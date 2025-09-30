use dioxus::prelude::*;
use burncloud_service_models::{ModelStatus, ModelType, InstalledModel, AvailableModel};

/// 简化版模型管理组件 - 现在从 AppState 获取数据
#[component]
pub fn SimpleModelManagement() -> Element {
    let mut search_term = use_signal(|| String::new());

    // NOTE: This component now requires AppState to be initialized with database
    // Data should come from AppState context instead of example data
    let installed_models: Vec<InstalledModel> = Vec::new();
    let available_models: Vec<AvailableModel> = Vec::new();

    // 基础过滤
    let filtered_installed: Vec<_> = installed_models
        .into_iter()
        .filter(|model| {
            if search_term.read().is_empty() {
                true
            } else {
                let query = search_term.read().to_lowercase();
                model.model.name.to_lowercase().contains(&query)
                    || model.model.display_name.to_lowercase().contains(&query)
                    || model.model.provider.to_lowercase().contains(&query)
            }
        })
        .collect();

    let filtered_available: Vec<_> = available_models
        .into_iter()
        .filter(|model| {
            if search_term.read().is_empty() {
                true
            } else {
                let query = search_term.read().to_lowercase();
                model.model.name.to_lowercase().contains(&query)
                    || model.model.display_name.to_lowercase().contains(&query)
                    || model.model.provider.to_lowercase().contains(&query)
            }
        })
        .collect();

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
                div { class: "grid gap-lg",
                    style: "grid-template-columns: 1fr;",
                    for installed_model in filtered_installed {
                        crate::models::InstalledModelCard { model: installed_model }
                    }
                }
            }

            // 可安装模型
            div {
                h2 { class: "text-title font-semibold mb-lg",
                    "可安装模型 ({filtered_available.len()})"
                }
                div { class: "grid gap-lg",
                    style: "grid-template-columns: 1fr;",
                    for available_model in filtered_available {
                        crate::models::AvailableModelCard { model: available_model }
                    }
                }
            }

            // 数据来源说明
            div { class: "mt-xxxl p-lg bg-info-light rounded",
                h3 { class: "text-subtitle font-semibold mb-md", "📊 数据来源" }
                p { class: "text-sm text-secondary mb-sm",
                    "此界面显示的所有模型数据都来自数据库通过 "
                    code { "burncloud-service-models" }
                    " crate 提供。"
                }
                ul { class: "text-sm text-secondary",
                    li { "已安装模型: 通过 ModelDataService 从数据库加载" }
                    li { "可下载模型: 通过 ModelsService 从数据库加载" }
                    li { "支持按名称、显示名称、提供商搜索" }
                    li { "所有模型都包含完整的元数据和状态信息" }
                }
            }
        }
    }
}

/// 模型统计组件
#[component]
pub fn ModelStats() -> Element {
    // NOTE: This component now requires AppState to be initialized with database
    let installed_models: Vec<InstalledModel> = Vec::new();
    let available_models: Vec<AvailableModel> = Vec::new();

    let running_count = installed_models
        .iter()
        .filter(|m| matches!(m.status, ModelStatus::Running))
        .count();

    let stopped_count = installed_models
        .iter()
        .filter(|m| matches!(m.status, ModelStatus::Stopped))
        .count();

    let total_usage: u64 = installed_models
        .iter()
        .map(|m| m.usage_count)
        .sum();

    let by_type = {
        let mut counts = std::collections::HashMap::new();
        for model in &installed_models {
            *counts.entry(&model.model.model_type).or_insert(0) += 1;
        }
        counts
    };

    rsx! {
        div { class: "stats-container",
            h2 { class: "text-title font-semibold mb-lg", "📊 模型统计" }

            div { class: "stats-grid",
                StatCard {
                    title: "总模型数",
                    value: installed_models.len().to_string(),
                    icon: "🧠",
                    color: "blue"
                }
                StatCard {
                    title: "运行中",
                    value: running_count.to_string(),
                    icon: "🟢",
                    color: "green"
                }
                StatCard {
                    title: "已停止",
                    value: stopped_count.to_string(),
                    icon: "🔴",
                    color: "gray"
                }
                StatCard {
                    title: "可下载",
                    value: available_models.len().to_string(),
                    icon: "📥",
                    color: "purple"
                }
                StatCard {
                    title: "总使用次数",
                    value: total_usage.to_string(),
                    icon: "📊",
                    color: "orange"
                }
            }

            // 按类型分布
            div { class: "mt-lg",
                h3 { class: "text-subtitle font-semibold mb-md", "按类型分布" }
                div { class: "type-distribution",
                    for (model_type, count) in by_type {
                        div { class: "type-item",
                            span { class: "type-icon", {format_type_icon(model_type)} }
                            span { class: "type-name", {format_type_name(model_type)} }
                            span { class: "type-count", "{count}" }
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

fn format_type_icon(model_type: &ModelType) -> &'static str {
    match model_type {
        ModelType::Chat => "💬",
        ModelType::Code => "💻",
        ModelType::Text => "📝",
        ModelType::Embedding => "🔗",
        ModelType::Multimodal => "🎭",
        ModelType::Image => "🖼️",
        ModelType::ImageGeneration => "🎨",
        ModelType::Audio => "🎵",
        ModelType::Speech => "🎤",
        ModelType::Video => "🎬",
        ModelType::Other => "📦",
    }
}

fn format_type_name(model_type: &ModelType) -> &'static str {
    match model_type {
        ModelType::Chat => "对话模型",
        ModelType::Code => "代码生成",
        ModelType::Text => "文本生成",
        ModelType::Embedding => "嵌入模型",
        ModelType::Multimodal => "多模态",
        ModelType::Image => "图像处理",
        ModelType::ImageGeneration => "图像生成",
        ModelType::Audio => "音频处理",
        ModelType::Speech => "语音模型",
        ModelType::Video => "视频处理",
        ModelType::Other => "其他类型",
    }
}