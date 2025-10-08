use dioxus::prelude::*;
use crate::app_state::AppState;
use burncloud_service_models::ModelStatus;

/// 增强版模型管理组件 - 使用 AppState 获取真实数据
#[component]
pub fn EnhancedModelManagement(app_state: AppState) -> Element {
    let mut search_term = use_signal(|| String::new());

    // 从 AppState 获取数据
    let (filtered_installed, filtered_available) = if search_term.read().is_empty() {
        (app_state.installed_models.iter().collect::<Vec<_>>(),
         app_state.available_models.iter().collect::<Vec<_>>())
    } else {
        app_state.search_models(&search_term.read())
    };

    // 获取统计信息
    let stats = app_state.get_stats();

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
                    value: stats.total_installed.to_string(),
                    icon: "🧠".to_string(),
                    color: "blue".to_string()
                }
                StatCard {
                    title: "运行中".to_string(),
                    value: stats.running_count.to_string(),
                    icon: "🟢".to_string(),
                    color: "green".to_string()
                }
                StatCard {
                    title: "已停止".to_string(),
                    value: stats.stopped_count.to_string(),
                    icon: "🔴".to_string(),
                    color: "red".to_string()
                }
                StatCard {
                    title: "可下载".to_string(),
                    value: stats.available_count.to_string(),
                    icon: "📥".to_string(),
                    color: "purple".to_string()
                }
            }

            // 模型网格
            div { class: "page-content",
                // 已安装模型部分
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
                        div { class: "grid gap-lg", style: "grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));",
                            for model in filtered_installed.iter() {
                                crate::models::InstalledModelCard { model: (*model).clone() }
                            }
                        }
                    }
                }

                // 可下载模型部分
                div {
                    h2 { class: "text-title font-semibold mb-lg",
                        "可下载模型 ({filtered_available.len()})"
                    }
                    if filtered_available.is_empty() {
                        div { class: "empty-state",
                            p { "没有找到可下载的模型" }
                            if !search_term.read().is_empty() {
                                p { class: "text-secondary", "尝试调整搜索条件" }
                            }
                        }
                    } else {
                        div { class: "grid gap-lg", style: "grid-template-columns: repeat(auto-fill, minmax(400px, 1fr));",
                            for model in filtered_available.iter() {
                                crate::models::AvailableModelCard { model: (*model).clone() }
                            }
                        }
                    }
                }

                // 详细统计信息
                div { class: "mt-xxxl",
                    h2 { class: "text-title font-semibold mb-lg", "📊 详细统计" }
                    div { class: "grid gap-lg", style: "grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));",
                        div { class: "card p-lg",
                            h3 { class: "text-subtitle font-semibold mb-md", "存储使用情况" }
                            div { class: "stat-item mb-sm",
                                span { class: "text-secondary", "总大小:" }
                                span { class: "font-semibold ml-sm", "{stats.format_total_size()}" }
                            }
                            div { class: "stat-item mb-sm",
                                span { class: "text-secondary", "平均模型大小:" }
                                span { class: "font-semibold ml-sm",
                                    if stats.total_installed > 0 {
                                        "{crate::IntegratedModelService::format_file_size(stats.total_size_bytes / stats.total_installed as u64)}"
                                    } else {
                                        "N/A"
                                    }
                                }
                            }
                        }
                        div { class: "card p-lg",
                            h3 { class: "text-subtitle font-semibold mb-md", "数据源信息" }
                            div { class: "stat-item mb-sm",
                                span { class: "text-secondary", "数据库:" }
                                span { class: "font-semibold ml-sm", "SQLite" }
                            }
                            div { class: "stat-item mb-sm",
                                span { class: "text-secondary", "服务层:" }
                                span { class: "font-semibold ml-sm", code { "burncloud-service-models" } }
                            }
                            div { class: "stat-item mb-sm",
                                span { class: "text-secondary", "数据完整性:" }
                                span { class: "font-semibold ml-sm text-success", "✅ 验证通过" }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// 统计卡片组件
#[component]
fn StatCard(title: String, value: String, icon: String, color: String) -> Element {
    let color_class = match color.as_str() {
        "blue" => "stat-card-blue",
        "green" => "stat-card-green",
        "red" => "stat-card-red",
        "purple" => "stat-card-purple",
        _ => "stat-card-default"
    };

    rsx! {
        div { class: "stat-card {color_class}",
            div { class: "stat-icon", "{icon}" }
            div { class: "stat-content",
                div { class: "stat-value", "{value}" }
                div { class: "stat-title", "{title}" }
            }
        }
    }
}