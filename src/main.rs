// 简化的示例程序来测试模型管理功能

use std::path::PathBuf;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 BurnCloud 模型管理系统启动");

    // 1. 测试模型发现客户端
    println!("\n1. 测试模型发现功能");
    test_model_discovery().await?;

    // 2. 测试下载管理器
    println!("\n2. 测试下载管理器");
    test_download_manager().await?;

    // 3. 测试模型验证器
    println!("\n3. 测试模型验证器");
    test_model_validator().await?;

    println!("\n✅ 所有测试完成！");
    Ok(())
}

async fn test_model_discovery() -> Result<(), Box<dyn std::error::Error>> {
    use burncloud_client_models::{ModelDiscoveryClient, ModelSearchRequest, discovery::{ModelType, SortBy, SortOrder}};

    // 创建发现客户端（使用模拟的URL）
    let client = ModelDiscoveryClient::new("https://api.example.com".to_string())?;

    // 创建搜索请求
    let search_request = ModelSearchRequest {
        query: Some("qwen".to_string()),
        model_type: Some(ModelType::ChatCompletion),
        provider: Some("Alibaba".to_string()),
        page_size: Some(10),
        sort_by: Some(SortBy::Rating),
        sort_order: Some(SortOrder::Desc),
        ..Default::default()
    };

    println!("   📝 搜索请求配置: {:?}", search_request.query);

    // 由于这是演示，我们会得到网络错误，这是预期的
    match client.search_models(search_request).await {
        Ok(response) => {
            println!("   ✅ 找到 {} 个模型", response.models.len());
        }
        Err(_) => {
            println!("   ⚠️  网络错误（这是预期的，因为我们使用的是演示URL）");
        }
    }

    Ok(())
}

async fn test_download_manager() -> Result<(), Box<dyn std::error::Error>> {
    use burncloud_client_models::ModelDownloadManager;

    // 创建临时目录
    let temp_dir = std::env::temp_dir().join("burncloud_test");
    std::fs::create_dir_all(&temp_dir)?;

    // 创建下载管理器
    let download_manager = ModelDownloadManager::new(temp_dir.clone())?
        .with_max_concurrent(2);

    println!("   📁 下载目录: {}", temp_dir.display());

    // 测试获取已安装模型
    match download_manager.get_installed_models().await {
        Ok(installed) => {
            println!("   📦 已安装模型数量: {}", installed.len());
        }
        Err(e) => {
            println!("   ⚠️  获取已安装模型失败: {}", e);
        }
    }

    Ok(())
}

async fn test_model_validator() -> Result<(), Box<dyn std::error::Error>> {
    use burncloud_client_models::{ModelValidator, ValidationConfig};

    // 创建临时目录
    let temp_dir = std::env::temp_dir().join("burncloud_validator_test");
    std::fs::create_dir_all(&temp_dir)?;

    // 创建验证器
    let validator = ModelValidator::new(temp_dir.clone())?;

    println!("   🔍 验证器临时目录: {}", temp_dir.display());

    // 创建一个测试文件
    let test_file = temp_dir.join("test_model.txt");
    std::fs::write(&test_file, "这是一个测试模型文件内容")?;

    println!("   📄 创建测试文件: {}", test_file.display());

    // 快速验证
    match validator.quick_validate(&test_file).await {
        Ok(is_valid) => {
            println!("   ✅ 快速验证结果: {}", if is_valid { "有效" } else { "无效" });
        }
        Err(e) => {
            println!("   ❌ 验证错误: {}", e);
        }
    }

    // 完整验证
    let config = ValidationConfig {
        enable_checksum_verification: true,
        enable_malware_scanning: true,
        enable_format_validation: true,
        enable_dependency_check: false,
        enable_permission_check: true,
        strict_mode: false,
        timeout_seconds: 30,
        quarantine_suspicious_files: false,
    };

    match validator.validate_model(&test_file, None, config).await {
        Ok(result) => {
            println!("   ✅ 完整验证完成");
            println!("      有效性: {}", result.is_valid);
            println!("      检查项目数: {}", result.checks_performed.len());
            println!("      错误数: {}", result.errors.len());
            println!("      警告数: {}", result.warnings.len());

            // 显示检查详情
            for check in &result.checks_performed {
                println!("      - {:?}: {:?}", check.check_type, check.status);
            }
        }
        Err(e) => {
            println!("   ❌ 验证错误: {}", e);
        }
    }

    // 清理测试文件
    let _ = std::fs::remove_file(&test_file);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_functionality() {
        // 基本功能测试
        let temp_dir = std::env::temp_dir().join("burncloud_unit_test");

        // 测试下载管理器创建
        let download_manager = burncloud_client_models::ModelDownloadManager::new(temp_dir.clone());
        assert!(download_manager.is_ok());

        // 测试验证器创建
        let validator = burncloud_client_models::ModelValidator::new(temp_dir.clone());
        assert!(validator.is_ok());

        // 测试发现客户端创建
        let discovery_client = burncloud_client_models::ModelDiscoveryClient::new("https://test.com".to_string());
        assert!(discovery_client.is_ok());
    }
}