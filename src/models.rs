use dioxus::prelude::*;
use burncloud_service_models::{InstalledModel, AvailableModel, ModelStatus, ModelType};

#[component]
pub fn ModelManagement() -> Element {
    let mut search_term = use_signal(|| String::new());

    // NOTE: This component now requires AppState to be initialized with database
    // Data should come from AppState context instead of example data
    let installed_models = use_signal(|| Vec::<InstalledModel>::new());
    let available_models = use_signal(|| Vec::<AvailableModel>::new());

    rsx! {
        div { class: "page-header",
            div { class: "flex justify-between items-center",
                div {
                    h1 { class: "text-large-title font-bold text-primary m-0",
                        "模型管理"
                    }
                    p { class: "text-secondary m-0 mt-sm",
                        "管理和部署大语言模型"
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
                    "已安装模型 ({installed_models.read().len()})"
                }
                div { class: "grid gap-lg",
                    style: "grid-template-columns: 1fr;",
                    for installed_model in installed_models.read().iter() {
                        InstalledModelCard { model: installed_model.clone() }
                    }
                }
            }

            // 可安装模型
            div {
                h2 { class: "text-title font-semibold mb-lg",
                    "可安装模型 (官方仓库)"
                }
                div { class: "grid gap-lg",
                    style: "grid-template-columns: 1fr;",
                    for available_model in available_models.read().iter() {
                        AvailableModelCard { model: available_model.clone() }
                    }
                }
            }
        }
    }
}

#[component]
pub fn InstalledModelCard(model: InstalledModel) -> Element {
    let status_class = match model.status {
        ModelStatus::Running => "status-running",
        ModelStatus::Stopped => "status-stopped",
        ModelStatus::Starting => "status-starting",
        ModelStatus::Stopping => "status-stopping",
        ModelStatus::Error => "status-error",
        _ => "status-unknown",
    };

    let status_text = match model.status {
        ModelStatus::Running => "运行中",
        ModelStatus::Stopped => "已停止",
        ModelStatus::Starting => "启动中",
        ModelStatus::Stopping => "停止中",
        ModelStatus::Error => "错误",
        _ => "未知",
    };

    let type_icon = match model.model.model_type {
        ModelType::Chat => "🧠",
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
    };

    let type_display = match model.model.model_type {
        ModelType::Chat => "💬对话专用",
        ModelType::Code => "💻代码生成",
        ModelType::Text => "📝文本生成",
        ModelType::Embedding => "🔗嵌入模型",
        ModelType::Multimodal => "🎭多模态",
        ModelType::Image => "🖼️图像处理",
        ModelType::ImageGeneration => "🎨图像生成",
        ModelType::Audio => "🎵音频处理",
        ModelType::Speech => "🎤语音模型",
        ModelType::Video => "🎬视频处理",
        ModelType::Other => "📦其他类型",
    };

    let action_button = match model.status {
        ModelStatus::Running => rsx! {
            button { class: "btn btn-secondary", "停止" }
        },
        ModelStatus::Stopped => rsx! {
            button { class: "btn btn-primary", "启动" }
        },
        _ => rsx! {
            button { class: "btn btn-secondary", disabled: true, "{status_text}" }
        },
    };

    rsx! {
        div { class: "card model-card",
            div { class: "model-header",
                div { class: "model-title",
                    span { style: "font-size: 20px;", "{type_icon}" }
                    div {
                        div { class: "text-subtitle font-semibold", "{model.model.display_name}" }
                        div { class: "text-caption text-secondary",
                            {model.model.description.as_deref().unwrap_or(&model.model.name)}
                        }
                    }
                }
                div { class: "flex items-center gap-md",
                    span { class: "status-indicator {status_class}",
                        span { class: "status-dot" }
                        "{status_text}"
                    }
                    div { class: "model-actions",
                        {action_button}
                        button { class: "btn btn-subtle", "配置" }
                        button { class: "btn btn-subtle", "删除" }
                    }
                }
            }
            div { class: "model-details",
                div {
                    div { class: "metric-label", "版本" }
                    div { class: "font-medium", "{model.model.version}" }
                }
                div {
                    div { class: "metric-label", "大小" }
                    div { class: "font-medium", "{model.model.formatted_size()}" }
                }
                if let Some(port) = model.port {
                    div {
                        div { class: "metric-label", "端口" }
                        div { class: "font-medium", "{port}" }
                    }
                }
                div {
                    div { class: "metric-label", "使用次数" }
                    div { class: "font-medium", "{model.usage_count}" }
                }
                if let Some(rating) = model.model.rating {
                    div {
                        div { class: "metric-label", "评分" }
                        div { class: "font-medium", "⭐{rating}" }
                    }
                }
                div {
                    div { class: "metric-label", "类型" }
                    div { class: "font-medium", "{type_display}" }
                }
            }
        }
    }
}

#[component]
pub fn AvailableModelCard(model: AvailableModel) -> Element {
    let type_icon = match model.model.model_type {
        ModelType::Chat => "🧠",
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
    };

    let type_display = match model.model.model_type {
        ModelType::Chat => "💬对话专用",
        ModelType::Code => "💻代码生成",
        ModelType::Text => "📝文本生成",
        ModelType::Embedding => "🔗嵌入模型",
        ModelType::Multimodal => "🎭多模态",
        ModelType::Image => "🖼️图像处理",
        ModelType::ImageGeneration => "🎨图像生成",
        ModelType::Audio => "🎵音频处理",
        ModelType::Speech => "🎤语音模型",
        ModelType::Video => "🎬视频处理",
        ModelType::Other => "📦其他类型",
    };

    // 检查是否为最新版本（最近7天内更新）
    let is_latest = {
        let now = chrono::Utc::now();
        let days_diff = now.signed_duration_since(model.model.updated_at).num_days();
        days_diff <= 7
    };

    rsx! {
        div { class: "card model-card",
            div { class: "model-header",
                div { class: "model-title",
                    span { style: "font-size: 20px;", "{type_icon}" }
                    div {
                        div { class: "text-subtitle font-semibold", "{model.model.display_name}" }
                        div { class: "text-caption text-secondary",
                            {model.model.description.as_deref().unwrap_or(&model.model.name)}
                        }
                    }
                    if is_latest {
                        span { class: "text-caption",
                            style: "background: linear-gradient(45deg, #ff6b6b, #feca57); color: white; padding: 2px 6px; border-radius: 4px; margin-left: 8px;",
                            "🔥最新版本"
                        }
                    }
                }
                div { class: "model-actions",
                    button { class: "btn btn-primary", "下载" }
                    button { class: "btn btn-subtle", "详情" }
                }
            }
            div { class: "model-details",
                div {
                    div { class: "metric-label", "大小" }
                    div { class: "font-medium", "📊{model.model.formatted_size()}" }
                }
                if let Some(rating) = model.model.rating {
                    div {
                        div { class: "metric-label", "评分" }
                        div { class: "font-medium", "⭐{rating}" }
                    }
                }
                div {
                    div { class: "metric-label", "类型" }
                    div { class: "font-medium", "{type_display}" }
                }
                div {
                    div { class: "metric-label", "更新时间" }
                    div { class: "font-medium", "{model.model.updated_at.format(\"%Y-%m-%d\")}" }
                }
                div {
                    div { class: "metric-label", "下载次数" }
                    div { class: "font-medium", "{model.model.download_count}" }
                }
                div {
                    div { class: "metric-label", "提供商" }
                    div { class: "font-medium", "{model.model.provider}" }
                }
            }
        }
    }
}