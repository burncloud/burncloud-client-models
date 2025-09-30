use burncloud_service_models::{
    Model, InstalledModel, CreateModelRequest, UpdateModelRequest, ModelFilter,
    ModelsService, ServiceError, ModelType, ModelStatus, SizeCategory
};
use burncloud_database_core::Database;
use std::sync::Arc;
use uuid::Uuid;
use std::collections::HashMap;

/// Client-level service that integrates with the complete database backend
///
/// This service provides a client-friendly interface to the multi-layer
/// database system, handling UI state management and user interactions.
#[derive(Clone)]
pub struct IntegratedModelService {
    service: Arc<ModelsService>,
}

impl IntegratedModelService {
    /// Create a new integrated model service
    ///
    /// This initializes the complete database stack and provides a client interface.
    pub async fn new(database_path: Option<String>) -> Result<Self, ClientError> {
        let db_path = database_path.unwrap_or_else(|| {
            let home = std::env::var("HOME")
                .or_else(|_| std::env::var("USERPROFILE"))
                .unwrap_or_else(|_| ".".to_string());
            format!("{}/burncloud/models.db", home)
        });

        // Ensure directory exists
        if let Some(parent) = std::path::Path::new(&db_path).parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| ClientError::InitializationFailed(format!("Failed to create directory: {}", e)))?;
        }

        // Initialize database core
        let mut database = Database::new(&db_path);
        database.initialize().await
            .map_err(|e| ClientError::InitializationFailed(format!("Database initialization failed: {}", e)))?;

        let database = Arc::new(database);

        // Initialize service layer
        let service = Arc::new(ModelsService::new(database).await
            .map_err(|e| ClientError::InitializationFailed(format!("Service initialization failed: {}", e)))?);

        Ok(Self { service })
    }

    /// Create a new model
    pub async fn create_model(&self, request: CreateModelRequest) -> Result<Model, ClientError> {
        self.service.create_model(request).await
            .map_err(ClientError::ServiceError)
    }

    /// Get a model by ID
    pub async fn get_model(&self, id: Uuid) -> Result<Option<Model>, ClientError> {
        self.service.get_model(id).await
            .map_err(ClientError::ServiceError)
    }

    /// List all models with optional filtering
    pub async fn list_models(&self, filter: Option<ModelFilter>) -> Result<Vec<Model>, ClientError> {
        let filter = filter.unwrap_or_default();
        self.service.list_models(filter).await
            .map_err(ClientError::ServiceError)
    }

    /// Search models by query string
    pub async fn search_models(&self, query: &str, limit: Option<u32>) -> Result<Vec<Model>, ClientError> {
        let filter = ModelFilter {
            search: Some(query.to_string()),
            limit,
            ..Default::default()
        };
        self.service.list_models(filter).await
            .map_err(ClientError::ServiceError)
    }

    /// Update a model
    pub async fn update_model(&self, id: Uuid, request: UpdateModelRequest) -> Result<Model, ClientError> {
        self.service.update_model(id, request).await
            .map_err(ClientError::ServiceError)
    }

    /// Delete a model
    pub async fn delete_model(&self, id: Uuid) -> Result<bool, ClientError> {
        self.service.delete_model(id).await
            .map_err(ClientError::ServiceError)
    }

    /// Get all installed models
    pub async fn get_installed_models(&self) -> Result<Vec<InstalledModel>, ClientError> {
        self.service.get_installed_models().await
            .map_err(ClientError::ServiceError)
    }

    /// Install a model
    pub async fn install_model(&self, model_id: Uuid, install_path: String) -> Result<InstalledModel, ClientError> {
        self.service.install_model(model_id, install_path).await
            .map_err(ClientError::ServiceError)
    }

    /// Update model status
    pub async fn update_model_status(&self, model_id: Uuid, status: ModelStatus) -> Result<(), ClientError> {
        self.service.update_model_status(model_id, status).await
            .map_err(ClientError::ServiceError)
    }

    /// Get models filtered by type
    pub async fn get_models_by_type(&self, model_type: ModelType) -> Result<Vec<Model>, ClientError> {
        let filter = ModelFilter {
            model_type: Some(model_type),
            ..Default::default()
        };
        self.service.list_models(filter).await
            .map_err(ClientError::ServiceError)
    }

    /// Get models filtered by provider
    pub async fn get_models_by_provider(&self, provider: &str) -> Result<Vec<Model>, ClientError> {
        let filter = ModelFilter {
            provider: Some(provider.to_string()),
            ..Default::default()
        };
        self.service.list_models(filter).await
            .map_err(ClientError::ServiceError)
    }

    /// Get official models only
    pub async fn get_official_models(&self) -> Result<Vec<Model>, ClientError> {
        let filter = ModelFilter {
            is_official: Some(true),
            ..Default::default()
        };
        self.service.list_models(filter).await
            .map_err(ClientError::ServiceError)
    }

    /// Get service statistics
    pub async fn get_statistics(&self) -> Result<ClientModelStats, ClientError> {
        let stats = self.service.get_model_stats().await
            .map_err(ClientError::ServiceError)?;

        Ok(ClientModelStats {
            total_models: stats.total_models,
            installed_count: stats.installed_count,
            official_count: stats.official_count,
            running_count: stats.running_count,
            total_size_bytes: stats.total_size_bytes,
            models_by_type: stats.models_by_type,
        })
    }

    /// Get models grouped by size category
    pub async fn get_models_by_size(&self) -> Result<HashMap<SizeCategory, Vec<Model>>, ClientError> {
        let models = self.list_models(None).await?;
        let mut grouped = HashMap::new();

        for model in models {
            grouped.entry(model.size_category)
                .or_insert_with(Vec::new)
                .push(model);
        }

        Ok(grouped)
    }

    /// Get recently updated models
    pub async fn get_recent_models(&self, limit: u32) -> Result<Vec<Model>, ClientError> {
        let filter = ModelFilter {
            limit: Some(limit),
            ..Default::default()
        };
        self.service.list_models(filter).await
            .map_err(ClientError::ServiceError)
    }

    /// Validate model data before creation
    pub fn validate_create_request(&self, request: &CreateModelRequest) -> Result<(), ClientError> {
        if request.name.is_empty() {
            return Err(ClientError::ValidationFailed("Model name cannot be empty".to_string()));
        }

        if request.display_name.is_empty() {
            return Err(ClientError::ValidationFailed("Display name cannot be empty".to_string()));
        }

        if request.file_size == 0 {
            return Err(ClientError::ValidationFailed("File size must be greater than 0".to_string()));
        }

        if request.provider.is_empty() {
            return Err(ClientError::ValidationFailed("Provider cannot be empty".to_string()));
        }

        Ok(())
    }

    /// Format file size for display
    pub fn format_file_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        const THRESHOLD: f64 = 1024.0;

        if bytes == 0 {
            return "0 B".to_string();
        }

        let size = bytes as f64;
        let unit_index = (size.ln() / THRESHOLD.ln()).floor() as usize;
        let unit_index = unit_index.min(UNITS.len() - 1);

        let value = size / THRESHOLD.powi(unit_index as i32);

        if unit_index == 0 {
            format!("{} {}", bytes, UNITS[unit_index])
        } else {
            format!("{:.1} {}", value, UNITS[unit_index])
        }
    }

    /// Get model type display name
    pub fn get_model_type_display_name(model_type: &ModelType) -> &'static str {
        match model_type {
            ModelType::Chat => "Conversational",
            ModelType::Code => "Code Generation",
            ModelType::Text => "Text Generation",
            ModelType::Embedding => "Text Embedding",
            ModelType::Image => "Image Processing",
            ModelType::ImageGeneration => "Image Generation",
            ModelType::Audio => "Audio Processing",
            ModelType::Speech => "Speech Processing",
            ModelType::Video => "Video Processing",
            ModelType::Multimodal => "Multimodal",
            ModelType::Other => "Other",
        }
    }

    /// Get status color for UI
    pub fn get_status_color(status: &ModelStatus) -> &'static str {
        match status {
            ModelStatus::Running => "#10B981", // green
            ModelStatus::Starting => "#F59E0B", // yellow
            ModelStatus::Stopping => "#F59E0B", // yellow
            ModelStatus::Stopped => "#6B7280", // gray
            ModelStatus::Error => "#EF4444", // red
        }
    }

    /// Check if model can be started
    pub fn can_start_model(model: &InstalledModel) -> bool {
        matches!(model.status, ModelStatus::Stopped | ModelStatus::Error)
    }

    /// Check if model can be stopped
    pub fn can_stop_model(model: &InstalledModel) -> bool {
        matches!(model.status, ModelStatus::Running | ModelStatus::Starting)
    }
}

/// Client-level statistics
#[derive(Debug, Clone)]
pub struct ClientModelStats {
    pub total_models: usize,
    pub installed_count: usize,
    pub official_count: usize,
    pub running_count: usize,
    pub total_size_bytes: u64,
    pub models_by_type: HashMap<ModelType, usize>,
}

impl ClientModelStats {
    /// Get installation rate as percentage
    pub fn installation_rate(&self) -> f64 {
        if self.total_models == 0 {
            0.0
        } else {
            (self.installed_count as f64 / self.total_models as f64) * 100.0
        }
    }

    /// Get running rate as percentage of installed models
    pub fn running_rate(&self) -> f64 {
        if self.installed_count == 0 {
            0.0
        } else {
            (self.running_count as f64 / self.installed_count as f64) * 100.0
        }
    }

    /// Get total size in human-readable format
    pub fn total_size_formatted(&self) -> String {
        IntegratedModelService::format_file_size(self.total_size_bytes)
    }

    /// Get most popular model type
    pub fn most_popular_type(&self) -> Option<&ModelType> {
        self.models_by_type
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(model_type, _)| model_type)
    }
}

/// Client-level errors
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Service error: {0}")]
    ServiceError(#[from] ServiceError),

    #[error("Initialization failed: {0}")]
    InitializationFailed(String),

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Operation not allowed: {0}")]
    OperationNotAllowed(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl ClientError {
    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            ClientError::ServiceError(se) => match se {
                ServiceError::NotFound(_) => "The requested model was not found.".to_string(),
                ServiceError::Conflict(_) => "A model with this name already exists.".to_string(),
                ServiceError::Validation(msg) => format!("Invalid input: {}", msg),
                ServiceError::Unauthorized(_) => "You don't have permission to perform this action.".to_string(),
                _ => "An unexpected error occurred. Please try again.".to_string(),
            },
            ClientError::ValidationFailed(msg) => msg.clone(),
            ClientError::InitializationFailed(_) => "Failed to initialize the model service. Please check your configuration.".to_string(),
            ClientError::OperationNotAllowed(msg) => msg.clone(),
            ClientError::ResourceNotFound(msg) => format!("Resource not found: {}", msg),
            ClientError::IoError(_) => "A file system error occurred.".to_string(),
        }
    }

    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            ClientError::ServiceError(se) => match se {
                ServiceError::Database(_) => true,
                ServiceError::Internal(_) => true,
                _ => false,
            },
            ClientError::IoError(_) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_service_initialization() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db").to_string_lossy().to_string();

        let service = IntegratedModelService::new(Some(db_path)).await.unwrap();

        // Test basic functionality
        let models = service.list_models(None).await.unwrap();
        assert_eq!(models.len(), 0); // Should start empty

        let stats = service.get_statistics().await.unwrap();
        assert_eq!(stats.total_models, 0);
    }

    #[test]
    fn test_file_size_formatting() {
        assert_eq!(IntegratedModelService::format_file_size(0), "0 B");
        assert_eq!(IntegratedModelService::format_file_size(512), "512 B");
        assert_eq!(IntegratedModelService::format_file_size(1024), "1.0 KB");
        assert_eq!(IntegratedModelService::format_file_size(1536), "1.5 KB");
        assert_eq!(IntegratedModelService::format_file_size(1024 * 1024), "1.0 MB");
        assert_eq!(IntegratedModelService::format_file_size(1024 * 1024 * 1024), "1.0 GB");
    }

    #[tokio::test]
    async fn test_validation() {
        let service = IntegratedModelService::new(None).await.unwrap();

        let invalid_request = CreateModelRequest {
            name: "".to_string(), // Empty name should fail
            display_name: "Test".to_string(),
            version: "1.0.0".to_string(),
            model_type: ModelType::Chat,
            provider: "Test".to_string(),
            file_size: 1024,
            description: None,
            license: None,
            tags: vec![],
            languages: vec![],
            file_path: None,
            download_url: None,
            config: HashMap::new(),
            is_official: false,
        };

        assert!(service.validate_create_request(&invalid_request).is_err());
    }
}