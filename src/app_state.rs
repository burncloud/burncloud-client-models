use std::sync::Arc;
use std::collections::HashMap;
use burncloud_service_models::{InstalledModel, AvailableModel, ModelStatus, ModelType};
use crate::{IntegratedModelService, ClientError};

/// 应用全局状态
#[derive(Clone)]
pub struct AppState {
    pub service: Arc<IntegratedModelService>,
    pub installed_models: Vec<InstalledModel>,
    pub available_models: Vec<AvailableModel>,
    pub loading: bool,
    pub error: Option<String>,
}

// 手动实现PartialEq，忽略service字段
impl PartialEq for AppState {
    fn eq(&self, other: &Self) -> bool {
        self.installed_models == other.installed_models
            && self.available_models == other.available_models
            && self.loading == other.loading
            && self.error == other.error
    }
}

impl AppState {
    /// 创建新的应用状态
    pub async fn new() -> Result<Self, ClientError> {
        // 使用 IntegratedModelService 的默认数据库路径 ($HOME/burncloud/models.db)
        println!("🚀 AppState: 使用默认数据库路径初始化服务");
        let service = Arc::new(IntegratedModelService::new(None).await?);
        println!("✅ AppState: 数据库连接初始化成功");

        Ok(Self {
            service,
            installed_models: Vec::new(),
            available_models: Vec::new(),
            loading: false,
            error: None,
        })
    }

    /// 加载所有数据
    pub async fn load_data(&mut self) -> Result<(), ClientError> {
        self.loading = true;
        self.error = None;

        // 加载已安装模型
        println!("🔍 AppState: 正在从数据库加载已安装模型...");
        match self.service.get_installed_models().await {
            Ok(models) => {
                println!("📊 AppState: 数据库中找到 {} 个已安装模型", models.len());
                for model in &models {
                    println!("  - 已安装: {} (状态: {:?})", model.model.display_name, model.status);
                }
                self.installed_models = models;
            }
            Err(e) => {
                let error_msg = format!("加载已安装模型失败: {}", e);
                println!("❌ AppState: {}", error_msg);
                self.error = Some(error_msg);
                self.loading = false;
                return Err(e);
            }
        }

        // 加载可用模型（从真实数据库）
        match self.load_available_models().await {
            Ok(models) => self.available_models = models,
            Err(e) => {
                let error_msg = format!("加载可用模型失败: {}", e);
                println!("❌ AppState: {}", error_msg);
                self.error = Some(error_msg);
                self.loading = false;
                return Err(e);
            }
        }

        self.loading = false;
        Ok(())
    }

    /// 加载可用模型（从数据库获取真实数据）
    async fn load_available_models(&self) -> Result<Vec<AvailableModel>, ClientError> {
        println!("🔍 AppState: 正在从数据库加载可用模型...");

        // 获取数据库中的所有模型（不创建示例数据）
        let all_models = self.service.list_models(None).await?;

        println!("📊 AppState: 数据库中找到 {} 个模型", all_models.len());
        for model in &all_models {
            println!("  - 模型: {} ({})", model.display_name, model.name);
        }

        // 转换为 AvailableModel
        let available_models: Vec<AvailableModel> = all_models.into_iter()
            .map(|model| AvailableModel {
                model,
                is_downloadable: true,
                estimated_download_time: Some(std::time::Duration::from_secs(300)), // 5分钟
            })
            .collect();

        println!("✅ AppState: 转换完成，可用模型数量: {}", available_models.len());
        Ok(available_models)
    }

    /// 刷新数据
    pub async fn refresh(&mut self) -> Result<(), ClientError> {
        self.load_data().await
    }

    /// 根据状态过滤已安装模型
    pub fn get_models_by_status(&self, status: ModelStatus) -> Vec<&InstalledModel> {
        self.installed_models
            .iter()
            .filter(|model| model.status == status)
            .collect()
    }

    /// 根据类型过滤模型
    pub fn get_models_by_type(&self, model_type: ModelType) -> Vec<&AvailableModel> {
        self.available_models
            .iter()
            .filter(|model| model.model.model_type == model_type)
            .collect()
    }

    /// 搜索模型
    pub fn search_models(&self, query: &str) -> (Vec<&InstalledModel>, Vec<&AvailableModel>) {
        let query_lower = query.to_lowercase();

        let installed = self.installed_models
            .iter()
            .filter(|model| {
                model.model.name.to_lowercase().contains(&query_lower)
                    || model.model.display_name.to_lowercase().contains(&query_lower)
                    || model.model.provider.to_lowercase().contains(&query_lower)
            })
            .collect();

        let available = self.available_models
            .iter()
            .filter(|model| {
                model.model.name.to_lowercase().contains(&query_lower)
                    || model.model.display_name.to_lowercase().contains(&query_lower)
                    || model.model.provider.to_lowercase().contains(&query_lower)
            })
            .collect();

        (installed, available)
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> AppStats {
        let total_installed = self.installed_models.len();
        let running_count = self.get_models_by_status(ModelStatus::Running).len();
        let stopped_count = self.get_models_by_status(ModelStatus::Stopped).len();
        let available_count = self.available_models.len();

        let total_size = self.installed_models
            .iter()
            .map(|m| m.model.file_size)
            .sum();

        let mut models_by_type = HashMap::new();
        for model in &self.installed_models {
            *models_by_type.entry(model.model.model_type.clone()).or_insert(0) += 1;
        }

        AppStats {
            total_installed,
            running_count,
            stopped_count,
            available_count,
            total_size_bytes: total_size,
            models_by_type,
        }
    }
}

/// 应用统计信息
#[derive(Debug, Clone)]
pub struct AppStats {
    pub total_installed: usize,
    pub running_count: usize,
    pub stopped_count: usize,
    pub available_count: usize,
    pub total_size_bytes: u64,
    pub models_by_type: HashMap<ModelType, usize>,
}

impl AppStats {
    /// 格式化文件大小
    pub fn format_total_size(&self) -> String {
        crate::IntegratedModelService::format_file_size(self.total_size_bytes)
    }
}