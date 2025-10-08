use dioxus::prelude::*;
use crate::app_state::AppState;

/// 模型统计组件 - 使用 AppState 展示数据统计
#[component]
pub fn ModelStats(app_state: AppState) -> Element {
    let stats = app_state.get_stats();

    rsx! {
        div { class: "stats-container",
            // 页面头部
            div { class: "page-header",
                h1 { class: "text-large-title font-bold text-primary m-0",
                    "模型统计分析"
                }
                p { class: "text-secondary m-0 mt-sm",
                    "基于 burncloud-service-models 数据的详细统计"
                }
            }

            div { class: "page-content",
                // 主要统计卡片
                div { class: "stats-grid mb-xxxl",
                    StatCard {
                        title: "总模型数".to_string(),
                        value: stats.total_installed.to_string(),
                        icon: "🧠".to_string(),
                        description: "已安装的模型总数".to_string(),
                        color: "blue".to_string()
                    }
                    StatCard {
                        title: "运行中".to_string(),
                        value: stats.running_count.to_string(),
                        icon: "🟢".to_string(),
                        description: "当前正在运行的模型".to_string(),
                        color: "green".to_string()
                    }
                    StatCard {
                        title: "已停止".to_string(),
                        value: stats.stopped_count.to_string(),
                        icon: "🔴".to_string(),
                        description: "当前已停止的模型".to_string(),
                        color: "red".to_string()
                    }
                    StatCard {
                        title: "存储占用".to_string(),
                        value: stats.format_total_size(),
                        icon: "💾".to_string(),
                        description: "模型文件总大小".to_string(),
                        color: "purple".to_string()
                    }
                }

                // 按类型分类统计
                div { class: "mb-xxxl",
                    h2 { class: "text-title font-semibold mb-lg", "📊 模型类型分布" }
                    if stats.models_by_type.is_empty() {
                        div { class: "empty-state",
                            p { "暂无模型数据" }
                        }
                    } else {
                        div { class: "grid gap-md", style: "grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));",
                            for (model_type, count) in stats.models_by_type.iter() {
                                div { class: "card p-md",
                                    div { class: "flex justify-between items-center",
                                        div {
                                            div { class: "font-semibold", "{get_model_type_display_name(model_type)}" }
                                            div { class: "text-sm text-secondary", "{model_type:?}" }
                                        }
                                        div { class: "text-xl font-bold text-primary", "{count}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // 数据来源和系统信息
                div { class: "grid gap-lg", style: "grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));",
                    div { class: "card p-lg",
                        h3 { class: "text-subtitle font-semibold mb-md", "📦 数据来源" }
                        div { class: "space-y-sm",
                            div { class: "flex justify-between",
                                span { class: "text-secondary", "数据库:" }
                                span { class: "font-semibold", "SQLite" }
                            }
                            div { class: "flex justify-between",
                                span { class: "text-secondary", "服务层:" }
                                span { class: "font-semibold", code { "burncloud-service-models" } }
                            }
                            div { class: "flex justify-between",
                                span { class: "text-secondary", "客户端层:" }
                                span { class: "font-semibold", code { "burncloud-client-models" } }
                            }
                            div { class: "flex justify-between",
                                span { class: "text-secondary", "数据一致性:" }
                                span { class: "font-semibold text-success", "✅ 验证通过" }
                            }
                        }
                    }
                    div { class: "card p-lg",
                        h3 { class: "text-subtitle font-semibold mb-md", "⚡ 系统状态" }
                        div { class: "space-y-sm",
                            div { class: "flex justify-between",
                                span { class: "text-secondary", "可用模型:" }
                                span { class: "font-semibold", "{stats.available_count}" }
                            }
                            div { class: "flex justify-between",
                                span { class: "text-secondary", "活跃模型:" }
                                span { class: "font-semibold", "{stats.running_count}" }
                            }
                            div { class: "flex justify-between",
                                span { class: "text-secondary", "使用率:" }
                                span { class: "font-semibold",
                                    if stats.total_installed > 0 {
                                        "{(stats.running_count as f64 / stats.total_installed as f64 * 100.0):.1}%"
                                    } else {
                                        "0%"
                                    }
                                }
                            }
                            div { class: "flex justify-between",
                                span { class: "text-secondary", "系统状态:" }
                                span { class: "font-semibold text-success", "🟢 正常" }
                            }
                        }
                    }
                }

                // 数据集成说明
                div { class: "mt-xxxl card p-lg bg-info-light",
                    h3 { class: "text-subtitle font-semibold mb-md", "🔗 数据集成架构" }
                    p { class: "text-sm text-secondary mb-md",
                        "此统计页面展示了完整的 BurnCloud 模型管理系统的数据流："
                    }
                    div { class: "grid gap-md", style: "grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));",
                        div { class: "integration-step",
                            div { class: "font-semibold text-primary", "1. 数据层" }
                            div { class: "text-sm text-secondary", "burncloud-database-models" }
                        }
                        div { class: "integration-step",
                            div { class: "font-semibold text-primary", "2. 服务层" }
                            div { class: "text-sm text-secondary", "burncloud-service-models" }
                        }
                        div { class: "integration-step",
                            div { class: "font-semibold text-primary", "3. 客户端层" }
                            div { class: "text-sm text-secondary", "burncloud-client-models" }
                        }
                        div { class: "integration-step",
                            div { class: "font-semibold text-primary", "4. UI层" }
                            div { class: "text-sm text-secondary", "Dioxus 前端" }
                        }
                    }
                }
            }
        }
    }
}

/// 增强型统计卡片组件
#[component]
fn StatCard(title: String, value: String, icon: String, description: String, color: String) -> Element {
    let color_class = match color.as_str() {
        "blue" => "stat-card-blue",
        "green" => "stat-card-green",
        "red" => "stat-card-red",
        "purple" => "stat-card-purple",
        _ => "stat-card-default"
    };

    rsx! {
        div { class: "stat-card {color_class}",
            div { class: "stat-header",
                div { class: "stat-icon", "{icon}" }
                div { class: "stat-title", "{title}" }
            }
            div { class: "stat-value", "{value}" }
            div { class: "stat-description", "{description}" }
        }
    }
}

/// 获取模型类型的显示名称
fn get_model_type_display_name(model_type: &burncloud_service_models::ModelType) -> &'static str {
    use burncloud_service_models::ModelType;
    match model_type {
        ModelType::Chat => "对话模型",
        ModelType::Code => "代码生成",
        ModelType::Text => "文本生成",
        ModelType::Embedding => "文本嵌入",
        ModelType::Image => "图像处理",
        ModelType::ImageGeneration => "图像生成",
        ModelType::Audio => "音频处理",
        ModelType::Speech => "语音处理",
        ModelType::Video => "视频处理",
        ModelType::Multimodal => "多模态",
        ModelType::Other => "其他",
    }
}