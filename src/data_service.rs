use burncloud_service_models::{
    InstalledModel, ModelStatus, ModelType, AvailableModel, RuntimeConfig
};
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ModelRuntime {
    pub model_id: Uuid,
    pub status: ModelStatus,
    pub port: Option<u32>,
    pub memory_usage_mb: u64,
    pub requests_per_second: f32,
}

#[derive(Debug, Clone)]
pub struct RuntimeMetrics {
    pub total_requests: u64,
    pub average_response_time_ms: f32,
    pub error_rate: f32,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone)]
pub struct SystemRequirements {
    pub min_memory_gb: f32,
    pub min_disk_space_gb: f32,
    pub gpu_required: bool,
    pub cpu_cores: u32,
}

/// 模型数据服务 - 提供模型数据的增删改查功能
#[derive(Clone)]
pub struct ModelDataService {
    installed_models: Vec<InstalledModel>,
    available_models: Vec<AvailableModel>,
    runtime_configs: Vec<RuntimeConfig>,
}

impl ModelDataService {
    /// 创建新的数据服务实例
    pub fn new() -> Self {
        Self {
            installed_models: crate::examples::get_example_installed_models(),
            available_models: crate::examples::get_example_available_models(),
            runtime_configs: crate::examples::get_example_runtime_configs(),
        }
    }

    /// 获取所有已安装模型
    pub fn get_installed_models(&self) -> &Vec<InstalledModel> {
        &self.installed_models
    }

    /// 获取所有可用模型
    pub fn get_available_models(&self) -> &Vec<AvailableModel> {
        &self.available_models
    }

    /// 根据状态筛选已安装模型
    pub fn get_installed_models_by_status(&self, status: &ModelStatus) -> Vec<&InstalledModel> {
        self.installed_models
            .iter()
            .filter(|model| &model.status == status)
            .collect()
    }

    /// 根据类型筛选模型
    pub fn get_models_by_type(&self, model_type: &ModelType) -> Vec<&InstalledModel> {
        self.installed_models
            .iter()
            .filter(|model| &model.model.model_type == model_type)
            .collect()
    }

    /// 搜索模型（按名称、描述等）
    pub fn search_models(&self, query: &str) -> Vec<&InstalledModel> {
        let query_lower = query.to_lowercase();
        self.installed_models
            .iter()
            .filter(|model| {
                model.model.name.to_lowercase().contains(&query_lower)
                    || model.model.display_name.to_lowercase().contains(&query_lower)
                    || model.model.description
                        .as_ref()
                        .map(|desc| desc.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
                    || model.model.provider.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// 根据ID获取已安装模型
    pub fn get_installed_model_by_id(&self, id: &Uuid) -> Option<&InstalledModel> {
        self.installed_models
            .iter()
            .find(|model| &model.model.id == id)
    }

    /// 根据ID获取可用模型
    pub fn get_available_model_by_id(&self, id: &Uuid) -> Option<&AvailableModel> {
        self.available_models
            .iter()
            .find(|model| &model.model.id == id)
    }

    /// 安装模型（从可用模型列表）
    pub fn install_model(&mut self, model_id: &Uuid, install_path: String) -> Result<(), String> {
        // 查找可用模型
        let available_model = self.get_available_model_by_id(model_id)
            .ok_or("模型不存在")?;

        // 检查是否已安装
        if self.get_installed_model_by_id(model_id).is_some() {
            return Err("模型已安装".to_string());
        }

        // 创建已安装模型
        let installed_model = InstalledModel {
            id: Uuid::new_v4(),
            model: available_model.model.clone(),
            install_path,
            installed_at: Utc::now(),
            status: ModelStatus::Stopped,
            port: None,
            process_id: None,
            last_used: None,
            usage_count: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.installed_models.push(installed_model);

        Ok(())
    }

    /// 卸载模型
    pub fn uninstall_model(&mut self, model_id: &Uuid) -> Result<(), String> {
        // 查找并删除已安装模型
        let index = self.installed_models
            .iter()
            .position(|model| &model.model.id == model_id)
            .ok_or("模型未安装")?;

        let removed_model = self.installed_models.remove(index);

        // 检查模型是否在运行
        if matches!(removed_model.status, ModelStatus::Running) {
            return Err("请先停止模型再卸载".to_string());
        }

        Ok(())
    }

    /// 启动模型
    pub fn start_model(&mut self, model_id: &Uuid, port: u16) -> Result<(), String> {
        // 先检查端口是否被占用
        if self.installed_models
            .iter()
            .any(|m| m.port == Some(port) && matches!(m.status, ModelStatus::Running))
        {
            return Err(format!("端口 {} 已被占用", port));
        }

        let model = self.installed_models
            .iter_mut()
            .find(|model| &model.model.id == model_id)
            .ok_or("模型未安装")?;

        match model.status {
            ModelStatus::Running => return Err("模型已在运行".to_string()),
            ModelStatus::Starting => return Err("模型正在启动".to_string()),
            _ => {}
        }

        model.status = ModelStatus::Starting;
        model.port = Some(port);

        // 模拟启动过程
        model.status = ModelStatus::Running;
        model.process_id = Some((rand::random::<u64>() % 65536 + 1000) as u32); // 模拟进程ID

        Ok(())
    }

    /// 停止模型
    pub fn stop_model(&mut self, model_id: &Uuid) -> Result<(), String> {
        let model = self.installed_models
            .iter_mut()
            .find(|model| &model.model.id == model_id)
            .ok_or("模型未安装")?;

        match model.status {
            ModelStatus::Stopped => return Err("模型已停止".to_string()),
            ModelStatus::Stopping => return Err("模型正在停止".to_string()),
            _ => {}
        }

        model.status = ModelStatus::Stopping;

        // 模拟停止过程
        model.status = ModelStatus::Stopped;
        model.process_id = None;

        Ok(())
    }

    /// 更新模型使用统计
    pub fn update_model_usage(&mut self, model_id: &Uuid) {
        if let Some(model) = self.installed_models
            .iter_mut()
            .find(|model| &model.model.id == model_id)
        {
            model.mark_used();
        }
    }

    /// 获取运行中的模型数量
    pub fn get_running_models_count(&self) -> usize {
        self.installed_models
            .iter()
            .filter(|model| matches!(model.status, ModelStatus::Running))
            .count()
    }

    /// 获取总的模型使用统计
    pub fn get_usage_stats(&self) -> ModelUsageStats {
        let total_models = self.installed_models.len();
        let running_models = self.get_running_models_count();
        let total_usage = self.installed_models
            .iter()
            .map(|model| model.usage_count)
            .sum();

        let models_by_type = self.installed_models
            .iter()
            .fold(HashMap::new(), |mut acc, model| {
                *acc.entry(model.model.model_type.clone()).or_insert(0) += 1;
                acc
            });

        ModelUsageStats {
            total_models,
            running_models,
            stopped_models: total_models - running_models,
            total_usage_count: total_usage,
            models_by_type,
        }
    }

    /// 获取系统资源使用概览
    pub fn get_resource_overview(&self) -> ResourceOverview {
        let total_disk_usage: u64 = self.installed_models
            .iter()
            .map(|model| model.model.file_size)
            .sum();

        let ports_in_use: Vec<u16> = self.installed_models
            .iter()
            .filter_map(|model| model.port)
            .collect();

        ResourceOverview {
            total_disk_usage_bytes: total_disk_usage,
            ports_in_use,
            active_processes: self.installed_models
                .iter()
                .filter_map(|model| model.process_id)
                .collect(),
        }
    }
}

impl Default for ModelDataService {
    fn default() -> Self {
        Self::new()
    }
}

/// 模型使用统计
#[derive(Debug, Clone, PartialEq)]
pub struct ModelUsageStats {
    pub total_models: usize,
    pub running_models: usize,
    pub stopped_models: usize,
    pub total_usage_count: u64,
    pub models_by_type: HashMap<ModelType, usize>,
}

/// 资源使用概览
#[derive(Debug, Clone)]
pub struct ResourceOverview {
    pub total_disk_usage_bytes: u64,
    pub ports_in_use: Vec<u16>,
    pub active_processes: Vec<u32>,
}

// 添加 rand 功能用于模拟
mod rand {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    pub fn random<T: From<u64>>() -> T {
        let mut hasher = DefaultHasher::new();
        std::time::SystemTime::now().hash(&mut hasher);
        T::from(hasher.finish())
    }
}