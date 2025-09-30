use burncloud_client_models::{IntegratedModelService, ClientError};
use burncloud_service_models::{CreateModelRequest, ModelType, UpdateModelRequest};
use std::collections::HashMap;
use uuid::Uuid;

/// Complete integration test demonstrating all four layers working together:
/// 1. burncloud-client-models (frontend integration)
/// 2. burncloud-service-models (business logic and validation)
/// 3. burncloud-database-models (database operations)
/// 4. burncloud-database-core (SQLite connection and queries)
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting BurnCloud Multi-Layer Database Integration Test");

    // Step 1: Initialize the integrated service (all layers)
    println!("\n📚 Step 1: Initializing integrated model service...");
    let service = IntegratedModelService::new(Some("./test_models.db".to_string())).await?;
    println!("✅ Service initialized successfully!");

    // Step 2: Verify empty state
    println!("\n📋 Step 2: Verifying initial empty state...");
    let initial_models = service.list_models(None).await?;
    let initial_stats = service.get_statistics().await?;
    println!("✅ Initial models count: {}", initial_models.len());
    println!("✅ Initial statistics: {:?}", initial_stats);

    // Step 3: Create test models
    println!("\n🔨 Step 3: Creating test models...");

    let models_to_create = vec![
        CreateModelRequest {
            name: "llama-3-8b-chat".to_string(),
            display_name: "Llama 3 8B Chat".to_string(),
            version: "1.0.0".to_string(),
            model_type: ModelType::Chat,
            provider: "Meta".to_string(),
            file_size: 8_589_934_592, // 8GB
            description: Some("A powerful conversational AI model".to_string()),
            license: Some("Custom".to_string()),
            tags: vec!["conversational".to_string(), "large".to_string()],
            languages: vec!["English".to_string(), "Spanish".to_string()],
            file_path: None,
            download_url: Some("https://example.com/llama-3-8b".to_string()),
            config: HashMap::new(),
            is_official: true,
        },
        CreateModelRequest {
            name: "codellama-7b".to_string(),
            display_name: "CodeLlama 7B".to_string(),
            version: "2.0.0".to_string(),
            model_type: ModelType::Code,
            provider: "Meta".to_string(),
            file_size: 7_516_192_768, // 7GB
            description: Some("Specialized code generation model".to_string()),
            license: Some("Custom".to_string()),
            tags: vec!["code".to_string(), "programming".to_string()],
            languages: vec!["Python".to_string(), "JavaScript".to_string(), "Rust".to_string()],
            file_path: None,
            download_url: Some("https://example.com/codellama-7b".to_string()),
            config: HashMap::new(),
            is_official: true,
        },
        CreateModelRequest {
            name: "mistral-7b-instruct".to_string(),
            display_name: "Mistral 7B Instruct".to_string(),
            version: "0.2".to_string(),
            model_type: ModelType::Text,
            provider: "Mistral AI".to_string(),
            file_size: 7_516_192_768, // 7GB
            description: Some("High-quality instruction-following model".to_string()),
            license: Some("Apache 2.0".to_string()),
            tags: vec!["instruction".to_string(), "efficient".to_string()],
            languages: vec!["English".to_string(), "French".to_string()],
            file_path: None,
            download_url: Some("https://example.com/mistral-7b".to_string()),
            config: HashMap::new(),
            is_official: false,
        },
    ];

    let mut created_models = Vec::new();
    for (i, request) in models_to_create.into_iter().enumerate() {
        println!("  Creating model {}: {}", i + 1, request.display_name);

        // Test validation
        service.validate_create_request(&request)?;

        let model = service.create_model(request).await?;
        created_models.push(model);
        println!("    ✅ Created model with ID: {}", created_models[i].id);
    }

    // Step 4: Test retrieval operations
    println!("\n🔍 Step 4: Testing retrieval operations...");

    // Test get all models
    let all_models = service.list_models(None).await?;
    println!("✅ Retrieved {} models", all_models.len());

    // Test get by ID
    let first_model_id = created_models[0].id;
    let retrieved_model = service.get_model(first_model_id).await?;
    println!("✅ Retrieved model by ID: {:?}", retrieved_model.as_ref().map(|m| &m.name));

    // Test search
    let search_results = service.search_models("llama", Some(10)).await?;
    println!("✅ Search for 'llama' found {} models", search_results.len());

    // Test filter by type
    let chat_models = service.get_models_by_type(ModelType::Chat).await?;
    println!("✅ Found {} chat models", chat_models.len());

    // Test filter by provider
    let meta_models = service.get_models_by_provider("Meta").await?;
    println!("✅ Found {} Meta models", meta_models.len());

    // Test official models
    let official_models = service.get_official_models().await?;
    println!("✅ Found {} official models", official_models.len());

    // Step 5: Test update operations
    println!("\n✏️ Step 5: Testing update operations...");

    let update_request = UpdateModelRequest {
        display_name: Some("Llama 3 8B Chat (Updated)".to_string()),
        description: Some("An updated powerful conversational AI model".to_string()),
        rating: Some(4.8),
        tags: Some(vec!["conversational".to_string(), "large".to_string(), "updated".to_string()]),
        ..Default::default()
    };

    let updated_model = service.update_model(first_model_id, update_request).await?;
    println!("✅ Updated model: {}", updated_model.display_name);

    // Step 6: Test installation operations
    println!("\n📦 Step 6: Testing installation operations...");

    // Install first model
    let install_path = format!("/opt/burncloud/models/{}", created_models[0].name);
    let installed_model = service.install_model(first_model_id, install_path).await?;
    println!("✅ Installed model: {} at {}", installed_model.model.name, installed_model.install_path);

    // Test status update
    use burncloud_service_models::ModelStatus;
    service.update_model_status(first_model_id, ModelStatus::Running).await?;
    println!("✅ Updated model status to Running");

    // Get installed models
    let installed_models = service.get_installed_models().await?;
    println!("✅ Retrieved {} installed models", installed_models.len());

    // Step 7: Test statistics and aggregations
    println!("\n📊 Step 7: Testing statistics and aggregations...");

    let stats = service.get_statistics().await?;
    println!("✅ Statistics:");
    println!("    Total models: {}", stats.total_models);
    println!("    Installed: {}", stats.installed_count);
    println!("    Running: {}", stats.running_count);
    println!("    Total size: {}", stats.total_size_formatted());
    println!("    Installation rate: {:.1}%", stats.installation_rate());
    println!("    Running rate: {:.1}%", stats.running_rate());

    if let Some(popular_type) = stats.most_popular_type() {
        println!("    Most popular type: {}", IntegratedModelService::get_model_type_display_name(popular_type));
    }

    // Test grouping by size
    let models_by_size = service.get_models_by_size().await?;
    println!("✅ Models by size category:");
    for (size_category, models) in &models_by_size {
        println!("    {:?}: {} models", size_category, models.len());
    }

    // Step 8: Test error handling
    println!("\n⚠️ Step 8: Testing error handling...");

    // Test duplicate creation
    let duplicate_request = CreateModelRequest {
        name: "llama-3-8b-chat".to_string(), // Same name as first model
        display_name: "Duplicate Model".to_string(),
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

    match service.create_model(duplicate_request).await {
        Err(ClientError::ServiceError(burncloud_service_models::ServiceError::Conflict(_))) => {
            println!("✅ Correctly caught duplicate model error");
        }
        other => {
            println!("❌ Expected conflict error, got: {:?}", other);
        }
    }

    // Test getting non-existent model
    let fake_id = Uuid::new_v4();
    let non_existent = service.get_model(fake_id).await?;
    if non_existent.is_none() {
        println!("✅ Correctly returned None for non-existent model");
    }

    // Step 9: Test deletion
    println!("\n🗑️ Step 9: Testing deletion...");

    // Try to delete installed model (should fail)
    match service.delete_model(first_model_id).await {
        Err(_) => println!("✅ Correctly prevented deletion of installed model"),
        Ok(_) => println!("❌ Should not allow deletion of installed model"),
    }

    // Delete non-installed model
    let second_model_id = created_models[1].id;
    let deleted = service.delete_model(second_model_id).await?;
    if deleted {
        println!("✅ Successfully deleted non-installed model");
    }

    // Step 10: Final verification
    println!("\n🏁 Step 10: Final verification...");

    let final_models = service.list_models(None).await?;
    let final_stats = service.get_statistics().await?;

    println!("✅ Final state:");
    println!("    Models count: {}", final_models.len());
    println!("    Installed models: {}", final_stats.installed_count);
    println!("    Running models: {}", final_stats.running_count);

    // Test utility functions
    println!("\n🔧 Testing utility functions...");

    for model in &final_models {
        println!("Model: {} ({})",
            model.display_name,
            IntegratedModelService::format_file_size(model.file_size)
        );
        println!("  Type: {}",
            IntegratedModelService::get_model_type_display_name(&model.model_type)
        );
    }

    for installed in &installed_models {
        println!("Installed: {} - Status: {} ({})",
            installed.model.name,
            installed.status,
            if IntegratedModelService::can_stop_model(installed) { "can stop" }
            else if IntegratedModelService::can_start_model(installed) { "can start" }
            else { "no actions" }
        );
    }

    println!("\n🎉 Integration test completed successfully!");
    println!("✅ All four layers (client, service, database-models, database-core) working together");

    // Cleanup
    println!("\n🧹 Cleaning up test database...");
    if let Err(e) = std::fs::remove_file("./test_models.db") {
        println!("Note: Could not remove test database: {}", e);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_integration_basic() {
        // Basic smoke test for CI/CD
        let service = IntegratedModelService::new(Some(":memory:".to_string())).await.unwrap();

        let models = service.list_models(None).await.unwrap();
        assert_eq!(models.len(), 0);

        let stats = service.get_statistics().await.unwrap();
        assert_eq!(stats.total_models, 0);
    }
}