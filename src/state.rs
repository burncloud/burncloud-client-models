use dioxus::prelude::*;
use burncloud_service_models::{InstalledModel, ModelStatus, ModelType, AvailableModel};
use burncloud_database::Database;
use crate::data_service::{ModelDataService, ModelUsageStats, ResourceOverview};
use uuid::Uuid;
use std::sync::Arc;

/// 全局应用状态管理
#[derive(Clone)]
pub struct AppState {
    pub data_service: ModelDataService,
    pub database: Arc<Database>,
    pub selected_model: Option<Uuid>,
    pub search_query: String,
    pub filter_type: Option<ModelType>,
    pub filter_status: Option<ModelStatus>,
}

impl AppState {
    pub async fn new(database: Arc<Database>) -> Result<Self, Box<dyn std::error::Error>> {
        let data_service = ModelDataService::new(database.clone()).await?;

        Ok(Self {
            data_service,
            database,
            selected_model: None,
            search_query: String::new(),
            filter_type: None,
            filter_status: None,
        })
    }

    /// 获取过滤后的已安装模型
    pub fn get_filtered_installed_models(&self) -> Vec<&InstalledModel> {
        let mut models: Vec<&InstalledModel> = if self.search_query.is_empty() {
            self.data_service.get_installed_models().iter().collect()
        } else {
            self.data_service.search_models(&self.search_query)
        };

        // 按类型过滤
        if let Some(filter_type) = &self.filter_type {
            models.retain(|model| &model.model.model_type == filter_type);
        }

        // 按状态过滤
        if let Some(filter_status) = &self.filter_status {
            models.retain(|model| &model.status == filter_status);
        }

        models
    }

    /// 获取过滤后的可用模型
    pub fn get_filtered_available_models(&self) -> Vec<&AvailableModel> {
        let mut models: Vec<&AvailableModel> =
            self.data_service.get_available_models().iter().collect();

        // 按搜索词过滤
        if !self.search_query.is_empty() {
            let query_lower = self.search_query.to_lowercase();
            models.retain(|model| {
                model.model.name.to_lowercase().contains(&query_lower)
                    || model.model.display_name.to_lowercase().contains(&query_lower)
                    || model.model.description
                        .as_ref()
                        .map(|desc| desc.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
                    || model.model.provider.to_lowercase().contains(&query_lower)
            });
        }

        // 按类型过滤
        if let Some(filter_type) = &self.filter_type {
            models.retain(|model| &model.model.model_type == filter_type);
        }

        models
    }

    /// 获取使用统计
    pub fn get_usage_stats(&self) -> ModelUsageStats {
        self.data_service.get_usage_stats()
    }

    /// 获取资源概览
    pub fn get_resource_overview(&self) -> ResourceOverview {
        self.data_service.get_resource_overview()
    }
}

/// 应用状态钩子
pub fn use_app_state() -> Signal<AppState> {
    use_context::<Signal<AppState>>()
}

/// 应用状态提供者组件
/// 注意: 此组件需要在实际使用时通过 use_resource 或 use_effect 进行异步初始化
/// 需要传入 Arc<Database> 来初始化 AppState
/// 由于 Arc<Database> 不实现 PartialEq，这不是一个标准的 Dioxus 组件
/// 请在应用代码中手动创建 AppState 并使用 use_context_provider
pub fn create_app_state_provider(_database: Arc<Database>) -> impl Fn(Element) -> Element {
    move |children: Element| {
        // This signature has been updated to accept database parameter
        // Actual async initialization should be handled in the consuming code
        // using use_resource or similar async initialization patterns
        let state = use_signal(|| None::<AppState>);

        use_context_provider(|| state);

        rsx! {
            {children}
        }
    }
}

/// 模型操作钩子 - 返回 Signal 而不是函数指针
pub fn use_model_actions() -> Signal<AppState> {
    use_app_state()
}

/// 模型操作接口
pub struct ModelActions {
    pub install_model: fn(Uuid, String) -> Result<(), String>,
    pub uninstall_model: fn(Uuid) -> Result<(), String>,
    pub start_model: fn(Uuid, u16) -> Result<(), String>,
    pub stop_model: fn(Uuid) -> Result<(), String>,
    pub update_usage: fn(Uuid),
    pub set_selected_model: fn(Option<Uuid>),
    pub set_search_query: fn(String),
    pub set_filter_type: fn(Option<ModelType>),
    pub set_filter_status: fn(Option<ModelStatus>),
}

/// 通知系统
#[derive(Debug, Clone, PartialEq)]
pub enum NotificationType {
    Success,
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub duration_ms: Option<u64>, // None = 永久显示
}

impl Notification {
    pub fn new(
        title: String,
        message: String,
        notification_type: NotificationType,
        duration_ms: Option<u64>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            message,
            notification_type,
            created_at: chrono::Utc::now(),
            duration_ms,
        }
    }

    pub fn success(title: String, message: String) -> Self {
        Self::new(title, message, NotificationType::Success, Some(3000))
    }

    pub fn error(title: String, message: String) -> Self {
        Self::new(title, message, NotificationType::Error, Some(5000))
    }

    pub fn warning(title: String, message: String) -> Self {
        Self::new(title, message, NotificationType::Warning, Some(4000))
    }

    pub fn info(title: String, message: String) -> Self {
        Self::new(title, message, NotificationType::Info, Some(3000))
    }
}

/// 通知状态管理
#[derive(Clone, Default)]
pub struct NotificationState {
    pub notifications: Vec<Notification>,
}

impl NotificationState {
    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }

    pub fn remove_notification(&mut self, id: &Uuid) {
        self.notifications.retain(|n| &n.id != id);
    }

    pub fn clear_all(&mut self) {
        self.notifications.clear();
    }
}

/// 通知钩子 - 简化版本
pub fn use_notifications() -> Signal<NotificationState> {
    use_signal(NotificationState::default)
}

/// 通知操作接口
pub struct NotificationActions {
    pub state: Signal<NotificationState>,
}