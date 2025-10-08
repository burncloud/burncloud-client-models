use dioxus::prelude::*;
use burncloud_service_models::{InstalledModel, AvailableModel, ModelStatus, ModelType};
use crate::app_state::AppState;

#[component]
pub fn ModelManagement() -> Element {
    let mut search_term = use_signal(|| String::new());
    let mut app_state = use_signal(|| None::<AppState>);
    let mut loading = use_signal(|| true);
    let mut error_message = use_signal(|| None::<String>);

    // 初始化应用状态并加载数据
    use_effect(move || {
        spawn(async move {
            println!("🚀 ModelManagement: 开始初始化数据服务...");
            match AppState::new().await {
                Ok(mut state) => {
                    println!("✅ ModelManagement: AppState 初始化成功");
                    match state.load_data().await {
                        Ok(_) => {
                            println!("✅ ModelManagement: 数据加载成功");
                            println!("📊 已安装模型数量: {}", state.installed_models.len());
                            println!("📊 可用模型数量: {}", state.available_models.len());
                            app_state.set(Some(state));
                        }
                        Err(e) => {
                            let error_msg = format!("数据加载失败: {}", e);
                            println!("❌ ModelManagement: {}", error_msg);
                            error_message.set(Some(error_msg));
                        }
                    }
                    loading.set(false);
                }
                Err(e) => {
                    let error_msg = format!("应用初始化失败: {}", e);
                    println!("❌ ModelManagement: {}", error_msg);
                    error_message.set(Some(error_msg));
                    loading.set(false);
                }
            }
        });
    });

    // 显示加载状态
    if *loading.read() {
        return rsx! {
            div { class: "page-content",
                style: "display: flex; justify-content: center; align-items: center; height: 400px; flex-direction: column;",
                div { class: "loading-spinner", style: "font-size: 24px; margin-bottom: 16px;", "🔄" }
                p { style: "color: #666; font-size: 16px;", "正在加载模型数据..." }
                p { style: "color: #999; font-size: 14px;", "首次加载可能需要几秒钟" }
            }
        };
    }

    // 显示错误状态
    if let Some(error) = error_message.read().as_ref() {
        return rsx! {
            div { class: "page-content",
                style: "display: flex; justify-content: center; align-items: center; height: 400px; flex-direction: column;",
                div { class: "error-icon", style: "font-size: 48px; margin-bottom: 16px;", "❌" }
                h2 { style: "color: #e74c3c; margin-bottom: 8px;", "数据加载失败" }
                p { style: "color: #666; margin-bottom: 16px;", "{error}" }
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        loading.set(true);
                        error_message.set(None);
                        // 重新加载数据
                    },
                    "重试"
                }
            }
        };
    }

    // 正常显示数据
    let state_ref = app_state.read();
    let state_option = state_ref.as_ref();

    match state_option {
        Some(state) => {
            let installed_models = &state.installed_models;
            let available_models = &state.available_models;

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
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| {
                                    loading.set(true);
                                    error_message.set(None);
                                    // 重新加载数据的逻辑
                                    spawn(async move {
                                        // 创建新的AppState实例并加载数据
                                        match AppState::new().await {
                                            Ok(mut new_state) => {
                                                match new_state.load_data().await {
                                                    Ok(_) => app_state.set(Some(new_state)),
                                                    Err(e) => error_message.set(Some(format!("{}", e))),
                                                }
                                            }
                                            Err(e) => error_message.set(Some(format!("{}", e))),
                                        }
                                        loading.set(false);
                                    });
                                },
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
                            "已安装模型 ({installed_models.len()})"
                        }
                        if installed_models.is_empty() {
                            div { class: "empty-state",
                                style: "text-align: center; padding: 40px; color: #666;",
                                div { style: "font-size: 48px; margin-bottom: 16px;", "📦" }
                                h3 { style: "margin-bottom: 8px;", "暂无已安装模型" }
                                p { "从下方的可安装模型列表中选择并安装模型" }
                            }
                        } else {
                            div { class: "grid gap-lg",
                                style: "grid-template-columns: 1fr;",
                                for installed_model in installed_models.iter() {
                                    InstalledModelCard { model: installed_model.clone() }
                                }
                            }
                        }
                    }

                    // 可安装模型
                    div {
                        h2 { class: "text-title font-semibold mb-lg",
                            "可安装模型 ({available_models.len()})"
                        }
                        if available_models.is_empty() {
                            div { class: "empty-state",
                                style: "text-align: center; padding: 40px; color: #666;",
                                div { style: "font-size: 48px; margin-bottom: 16px;", "🌐" }
                                h3 { style: "margin-bottom: 8px;", "暂无可安装模型" }
                                p { "请检查网络连接或稍后重试" }
                            }
                        } else {
                            div { class: "grid gap-lg",
                                style: "grid-template-columns: 1fr;",
                                for available_model in available_models.iter() {
                                    AvailableModelCard { model: available_model.clone() }
                                }
                            }
                        }
                    }
                }
            }
        }
        None => {
            rsx! {
                div { class: "page-content",
                    style: "display: flex; justify-content: center; align-items: center; height: 400px;",
                    p { style: "color: #666; font-size: 16px;", "数据未初始化" }
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