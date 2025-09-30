use burncloud_service_models::{
    Model, ModelType, ModelSize, ModelStatus, InstalledModel, AvailableModel,
    RuntimeConfig, SystemRequirements
};
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

/// 示例模型数据 - 已安装的模型
pub fn get_example_installed_models() -> Vec<InstalledModel> {
    vec![
        create_qwen_7b_model(),
        create_deepseek_v2_model(),
        create_code_llama_model(),
        create_embedding_model(),
    ]
}

/// 示例模型数据 - 可下载的模型
pub fn get_example_available_models() -> Vec<AvailableModel> {
    vec![
        create_qwen_14b_available(),
        create_deepseek_coder_v2_available(),
        create_qwen_32b_available(),
        create_llama_3_8b_available(),
        create_chatglm_6b_available(),
        create_baichuan_13b_available(),
    ]
}

/// 创建 Qwen2.5-7B-Chat 已安装模型示例
fn create_qwen_7b_model() -> InstalledModel {
    let model = Model {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
        name: "qwen2.5-7b-chat".to_string(),
        display_name: "Qwen2.5-7B-Chat".to_string(),
        description: Some("阿里巴巴通义千问 7B 对话模型".to_string()),
        version: "v1.2".to_string(),
        model_type: ModelType::Chat,
        size_category: ModelSize::Medium,
        file_size: 4_398_046_511_104, // 4.1GB
        provider: "Alibaba".to_string(),
        license: Some("Apache 2.0".to_string()),
        tags: vec!["chat".to_string(), "chinese".to_string(), "multilingual".to_string()],
        languages: vec!["zh".to_string(), "en".to_string()],
        created_at: Utc::now() - chrono::Duration::days(30),
        updated_at: Utc::now() - chrono::Duration::days(5),
        file_path: Some("/data/models/qwen2.5-7b-chat".to_string()),
        checksum: Some("sha256:abc123def456...".to_string()),
        download_url: Some("https://huggingface.co/Qwen/Qwen2.5-7B-Chat".to_string()),
        config: {
            let mut map = HashMap::new();
            map.insert("max_context_length".to_string(), serde_json::Value::Number(serde_json::Number::from(32768)));
            map.insert("temperature".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.7).unwrap()));
            map.insert("top_p".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.8).unwrap()));
            map
        },
        rating: Some(4.8),
        download_count: 125430,
        is_official: true,
    };

    let mut installed = InstalledModel::from_model(model, "/data/models/qwen2.5-7b-chat".to_string());
    installed.status = ModelStatus::Running;
    installed.port = Some(8001);
    installed.process_id = Some(12345);
    installed.last_used = Some(Utc::now() - chrono::Duration::hours(2));
    installed.usage_count = 342;
    installed
}

/// 创建 DeepSeek-V2-Chat 已安装模型示例
fn create_deepseek_v2_model() -> InstalledModel {
    let model = Model {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap(),
        name: "deepseek-v2-chat".to_string(),
        display_name: "DeepSeek-V2-Chat".to_string(),
        description: Some("DeepSeek 深度求索第二代对话模型".to_string()),
        version: "v2.0".to_string(),
        model_type: ModelType::Chat,
        size_category: ModelSize::Medium,
        file_size: 7_301_444_403_200, // 6.8GB
        provider: "DeepSeek".to_string(),
        license: Some("MIT".to_string()),
        tags: vec!["chat".to_string(), "code".to_string(), "reasoning".to_string()],
        languages: vec!["zh".to_string(), "en".to_string()],
        created_at: Utc::now() - chrono::Duration::days(45),
        updated_at: Utc::now() - chrono::Duration::days(10),
        file_path: Some("/data/models/deepseek-v2-chat".to_string()),
        checksum: Some("sha256:def456ghi789...".to_string()),
        download_url: Some("https://huggingface.co/deepseek-ai/DeepSeek-V2-Chat".to_string()),
        config: {
            let mut map = HashMap::new();
            map.insert("max_context_length".to_string(), serde_json::Value::Number(serde_json::Number::from(32768)));
            map.insert("temperature".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.7).unwrap()));
            map.insert("top_p".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.95).unwrap()));
            map
        },
        rating: Some(4.9),
        download_count: 89432,
        is_official: true,
    };

    let mut installed = InstalledModel::from_model(model, "/data/models/deepseek-v2-chat".to_string());
    installed.status = ModelStatus::Stopped;
    installed.port = Some(8002);
    installed.last_used = Some(Utc::now() - chrono::Duration::days(3));
    installed.usage_count = 156;
    installed
}

/// 创建 Code Llama 已安装模型示例
fn create_code_llama_model() -> InstalledModel {
    let model = Model {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440003").unwrap(),
        name: "code-llama-7b".to_string(),
        display_name: "Code Llama 7B".to_string(),
        description: Some("Meta 专业代码生成模型".to_string()),
        version: "v1.0".to_string(),
        model_type: ModelType::Code,
        size_category: ModelSize::Medium,
        file_size: 6_989_586_022_400, // 6.5GB
        provider: "Meta".to_string(),
        license: Some("Custom License".to_string()),
        tags: vec!["code".to_string(), "programming".to_string(), "llama".to_string()],
        languages: vec!["en".to_string()],
        created_at: Utc::now() - chrono::Duration::days(60),
        updated_at: Utc::now() - chrono::Duration::days(15),
        file_path: Some("/data/models/code-llama-7b".to_string()),
        checksum: Some("sha256:ghi789jkl012...".to_string()),
        download_url: Some("https://huggingface.co/codellama/CodeLlama-7b-hf".to_string()),
        config: {
            let mut map = HashMap::new();
            map.insert("max_context_length".to_string(), serde_json::Value::Number(serde_json::Number::from(16384)));
            map.insert("temperature".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.1).unwrap()));
            map.insert("top_p".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.95).unwrap()));
            map
        },
        rating: Some(4.7),
        download_count: 76541,
        is_official: true,
    };

    let mut installed = InstalledModel::from_model(model, "/data/models/code-llama-7b".to_string());
    installed.status = ModelStatus::Stopped;
    installed.port = Some(8003);
    installed.last_used = Some(Utc::now() - chrono::Duration::days(1));
    installed.usage_count = 89;
    installed
}

/// 创建嵌入模型示例
fn create_embedding_model() -> InstalledModel {
    let model = Model {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440004").unwrap(),
        name: "bge-large-zh".to_string(),
        display_name: "BGE Large Chinese".to_string(),
        description: Some("BAAI 中文文本嵌入模型".to_string()),
        version: "v1.5".to_string(),
        model_type: ModelType::Embedding,
        size_category: ModelSize::Small,
        file_size: 1_288_490_188_800, // 1.2GB
        provider: "BAAI".to_string(),
        license: Some("MIT".to_string()),
        tags: vec!["embedding".to_string(), "chinese".to_string(), "retrieval".to_string()],
        languages: vec!["zh".to_string()],
        created_at: Utc::now() - chrono::Duration::days(20),
        updated_at: Utc::now() - chrono::Duration::days(2),
        file_path: Some("/data/models/bge-large-zh".to_string()),
        checksum: Some("sha256:jkl012mno345...".to_string()),
        download_url: Some("https://huggingface.co/BAAI/bge-large-zh".to_string()),
        config: {
            let mut map = HashMap::new();
            map.insert("max_seq_length".to_string(), serde_json::Value::Number(serde_json::Number::from(512)));
            map.insert("normalize_embeddings".to_string(), serde_json::Value::Bool(true));
            map
        },
        rating: Some(4.6),
        download_count: 45623,
        is_official: true,
    };

    let mut installed = InstalledModel::from_model(model, "/data/models/bge-large-zh".to_string());
    installed.status = ModelStatus::Running;
    installed.port = Some(8004);
    installed.process_id = Some(23456);
    installed.last_used = Some(Utc::now() - chrono::Duration::minutes(30));
    installed.usage_count = 1234;
    installed
}

/// 创建 Qwen2.5-14B 可下载模型示例
fn create_qwen_14b_available() -> AvailableModel {
    let model = Model {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440005").unwrap(),
        name: "qwen2.5-14b-chat".to_string(),
        display_name: "Qwen2.5-14B-Chat".to_string(),
        description: Some("阿里巴巴通义千问 14B 对话模型".to_string()),
        version: "v1.0".to_string(),
        model_type: ModelType::Chat,
        size_category: ModelSize::Large,
        file_size: 7_731_814_400_000, // 7.2GB
        provider: "Alibaba".to_string(),
        license: Some("Apache 2.0".to_string()),
        tags: vec!["chat".to_string(), "chinese".to_string(), "latest".to_string()],
        languages: vec!["zh".to_string(), "en".to_string()],
        created_at: Utc::now() - chrono::Duration::days(5),
        updated_at: Utc::now() - chrono::Duration::days(1),
        file_path: None,
        checksum: Some("sha256:mno345pqr678...".to_string()),
        download_url: Some("https://huggingface.co/Qwen/Qwen2.5-14B-Chat".to_string()),
        config: std::collections::HashMap::new(),
        rating: Some(4.8),
        download_count: 12543,
        is_official: true,
    };

    let system_requirements = SystemRequirements {
        min_memory_gb: 8.0,
        recommended_memory_gb: 16.0,
        min_disk_space_gb: 10.0,
        requires_gpu: true,
        supported_os: vec!["linux".to_string(), "windows".to_string(), "macos".to_string()],
        supported_architectures: vec!["x86_64".to_string(), "arm64".to_string()],
    };

    AvailableModel::from_model(model, system_requirements)
}

/// 创建 DeepSeek-Coder-V2 可下载模型示例
fn create_deepseek_coder_v2_available() -> AvailableModel {
    let model = Model {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440006").unwrap(),
        name: "deepseek-coder-v2".to_string(),
        display_name: "DeepSeek-Coder-V2".to_string(),
        description: Some("DeepSeek 专业代码生成模型".to_string()),
        version: "v2.0".to_string(),
        model_type: ModelType::Code,
        size_category: ModelSize::Medium,
        file_size: 5_478_566_912_000, // 5.1GB
        provider: "DeepSeek".to_string(),
        license: Some("MIT".to_string()),
        tags: vec!["code".to_string(), "programming".to_string(), "latest".to_string()],
        languages: vec!["en".to_string()],
        created_at: Utc::now() - chrono::Duration::days(3),
        updated_at: Utc::now() - chrono::Duration::hours(12),
        file_path: None,
        checksum: Some("sha256:pqr678stu901...".to_string()),
        download_url: Some("https://huggingface.co/deepseek-ai/DeepSeek-Coder-V2".to_string()),
        config: std::collections::HashMap::new(),
        rating: Some(4.9),
        download_count: 8765,
        is_official: true,
    };

    let system_requirements = SystemRequirements {
        min_memory_gb: 6.0,
        recommended_memory_gb: 12.0,
        min_disk_space_gb: 8.0,
        requires_gpu: true,
        supported_os: vec!["linux".to_string(), "windows".to_string()],
        supported_architectures: vec!["x86_64".to_string()],
    };

    AvailableModel::from_model(model, system_requirements)
}

/// 创建 Qwen2.5-32B 可下载模型示例
fn create_qwen_32b_available() -> AvailableModel {
    let model = Model {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440007").unwrap(),
        name: "qwen2.5-32b-chat".to_string(),
        display_name: "Qwen2.5-32B-Chat".to_string(),
        description: Some("阿里巴巴通义千问 32B 大型对话模型".to_string()),
        version: "v1.0".to_string(),
        model_type: ModelType::Chat,
        size_category: ModelSize::XLarge,
        file_size: 19_863_132_364_800, // 18.5GB
        provider: "Alibaba".to_string(),
        license: Some("Apache 2.0".to_string()),
        tags: vec!["chat".to_string(), "chinese".to_string(), "latest".to_string(), "large".to_string()],
        languages: vec!["zh".to_string(), "en".to_string(), "ja".to_string(), "ko".to_string()],
        created_at: Utc::now() - chrono::Duration::days(1),
        updated_at: Utc::now() - chrono::Duration::hours(6),
        file_path: None,
        checksum: Some("sha256:stu901vwx234...".to_string()),
        download_url: Some("https://huggingface.co/Qwen/Qwen2.5-32B-Chat".to_string()),
        config: std::collections::HashMap::new(),
        rating: Some(4.9),
        download_count: 3421,
        is_official: true,
    };

    let system_requirements = SystemRequirements {
        min_memory_gb: 20.0,
        recommended_memory_gb: 32.0,
        min_disk_space_gb: 25.0,
        requires_gpu: true,
        supported_os: vec!["linux".to_string(), "windows".to_string()],
        supported_architectures: vec!["x86_64".to_string()],
    };

    AvailableModel::from_model(model, system_requirements)
}

/// 创建 Llama 3 8B 可下载模型示例
fn create_llama_3_8b_available() -> AvailableModel {
    let model = Model {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440008").unwrap(),
        name: "llama-3-8b-instruct".to_string(),
        display_name: "Llama 3 8B Instruct".to_string(),
        description: Some("Meta Llama 3 8B 指令调优模型".to_string()),
        version: "v3.0".to_string(),
        model_type: ModelType::Chat,
        size_category: ModelSize::Medium,
        file_size: 8_589_934_592_000, // 8.0GB
        provider: "Meta".to_string(),
        license: Some("Llama 3 License".to_string()),
        tags: vec!["chat".to_string(), "instruct".to_string(), "llama".to_string()],
        languages: vec!["en".to_string()],
        created_at: Utc::now() - chrono::Duration::days(15),
        updated_at: Utc::now() - chrono::Duration::days(7),
        file_path: None,
        checksum: Some("sha256:vwx234yza567...".to_string()),
        download_url: Some("https://huggingface.co/meta-llama/Meta-Llama-3-8B-Instruct".to_string()),
        config: std::collections::HashMap::new(),
        rating: Some(4.7),
        download_count: 54321,
        is_official: true,
    };

    let system_requirements = SystemRequirements {
        min_memory_gb: 10.0,
        recommended_memory_gb: 16.0,
        min_disk_space_gb: 12.0,
        requires_gpu: true,
        supported_os: vec!["linux".to_string(), "windows".to_string(), "macos".to_string()],
        supported_architectures: vec!["x86_64".to_string(), "arm64".to_string()],
    };

    AvailableModel::from_model(model, system_requirements)
}

/// 创建 ChatGLM 6B 可下载模型示例
fn create_chatglm_6b_available() -> AvailableModel {
    let model = Model {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440009").unwrap(),
        name: "chatglm3-6b".to_string(),
        display_name: "ChatGLM3-6B".to_string(),
        description: Some("清华大学 ChatGLM 第三代 6B 对话模型".to_string()),
        version: "v3.0".to_string(),
        model_type: ModelType::Chat,
        size_category: ModelSize::Medium,
        file_size: 6_442_450_944_000, // 6.0GB
        provider: "Tsinghua KEG".to_string(),
        license: Some("Apache 2.0".to_string()),
        tags: vec!["chat".to_string(), "chinese".to_string(), "chatglm".to_string()],
        languages: vec!["zh".to_string(), "en".to_string()],
        created_at: Utc::now() - chrono::Duration::days(25),
        updated_at: Utc::now() - chrono::Duration::days(12),
        file_path: None,
        checksum: Some("sha256:yza567bcd890...".to_string()),
        download_url: Some("https://huggingface.co/THUDM/chatglm3-6b".to_string()),
        config: std::collections::HashMap::new(),
        rating: Some(4.6),
        download_count: 32145,
        is_official: true,
    };

    let system_requirements = SystemRequirements {
        min_memory_gb: 8.0,
        recommended_memory_gb: 12.0,
        min_disk_space_gb: 8.0,
        requires_gpu: false,
        supported_os: vec!["linux".to_string(), "windows".to_string(), "macos".to_string()],
        supported_architectures: vec!["x86_64".to_string(), "arm64".to_string()],
    };

    AvailableModel::from_model(model, system_requirements)
}

/// 创建 Baichuan 13B 可下载模型示例
fn create_baichuan_13b_available() -> AvailableModel {
    let model = Model {
        id: Uuid::parse_str("550e8400-e29b-41d4-a716-446655440010").unwrap(),
        name: "baichuan2-13b-chat".to_string(),
        display_name: "Baichuan2-13B-Chat".to_string(),
        description: Some("百川智能第二代 13B 对话模型".to_string()),
        version: "v2.0".to_string(),
        model_type: ModelType::Chat,
        size_category: ModelSize::Large,
        file_size: 13_958_643_712_000, // 13.0GB
        provider: "Baichuan Inc.".to_string(),
        license: Some("Baichuan 2 License".to_string()),
        tags: vec!["chat".to_string(), "chinese".to_string(), "baichuan".to_string()],
        languages: vec!["zh".to_string(), "en".to_string()],
        created_at: Utc::now() - chrono::Duration::days(40),
        updated_at: Utc::now() - chrono::Duration::days(20),
        file_path: None,
        checksum: Some("sha256:bcd890efg123...".to_string()),
        download_url: Some("https://huggingface.co/baichuan-inc/Baichuan2-13B-Chat".to_string()),
        config: std::collections::HashMap::new(),
        rating: Some(4.5),
        download_count: 18976,
        is_official: true,
    };

    let system_requirements = SystemRequirements {
        min_memory_gb: 16.0,
        recommended_memory_gb: 24.0,
        min_disk_space_gb: 18.0,
        requires_gpu: true,
        supported_os: vec!["linux".to_string(), "windows".to_string()],
        supported_architectures: vec!["x86_64".to_string()],
    };

    AvailableModel::from_model(model, system_requirements)
}

/// 获取示例运行时配置
pub fn get_example_runtime_configs() -> Vec<RuntimeConfig> {
    vec![
        {
            let mut config = HashMap::new();
            config.insert("max_context_length".to_string(), serde_json::Value::Number(serde_json::Number::from(4096)));
            config.insert("temperature".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.7).unwrap()));
            config.insert("top_p".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.9).unwrap()));
            config.insert("top_k".to_string(), serde_json::Value::Number(serde_json::Number::from(50)));
            config.insert("max_tokens".to_string(), serde_json::Value::Number(serde_json::Number::from(2048)));
            config.insert("batch_size".to_string(), serde_json::Value::Number(serde_json::Number::from(1)));
            config.insert("max_concurrent_requests".to_string(), serde_json::Value::Number(serde_json::Number::from(10)));
            config.insert("memory_limit_mb".to_string(), serde_json::Value::Number(serde_json::Number::from(8192)));
            config.insert("enable_streaming".to_string(), serde_json::Value::Bool(true));
            config.insert("repetition_penalty".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(1.1).unwrap()));
            config.insert("do_sample".to_string(), serde_json::Value::Bool(true));
            config
        },
        {
            let mut config = HashMap::new();
            config.insert("max_context_length".to_string(), serde_json::Value::Number(serde_json::Number::from(32768)));
            config.insert("temperature".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.1).unwrap()));
            config.insert("top_p".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(0.95).unwrap()));
            config.insert("top_k".to_string(), serde_json::Value::Number(serde_json::Number::from(40)));
            config.insert("max_tokens".to_string(), serde_json::Value::Number(serde_json::Number::from(4096)));
            config.insert("batch_size".to_string(), serde_json::Value::Number(serde_json::Number::from(2)));
            config.insert("max_concurrent_requests".to_string(), serde_json::Value::Number(serde_json::Number::from(5)));
            config.insert("memory_limit_mb".to_string(), serde_json::Value::Number(serde_json::Number::from(16384)));
            config.insert("enable_streaming".to_string(), serde_json::Value::Bool(true));
            config.insert("code_mode".to_string(), serde_json::Value::Bool(true));
            config.insert("syntax_highlighting".to_string(), serde_json::Value::Bool(true));
            config
        },
    ]
}