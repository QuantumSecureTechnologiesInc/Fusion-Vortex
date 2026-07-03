use flux_resolve_v2_hive_mind::{cache::CacheLayer, FluxEngine, Manifest};

#[tokio::test]
async fn test_cache_operations() {
    let cache = CacheLayer::new();
    
    // Test put and get
    let hash = "integration_test_hash";
    let data = "LOCK_INTEGRATION_TEST";
    
    cache.put(hash, data, Some(60000)).await.unwrap();
    let retrieved = cache.get(hash).await.unwrap();
    
    assert_eq!(retrieved, Some(data.to_string()));
}

#[tokio::test]
async fn test_engine_resolve() {
    let cache = std::sync::Arc::new(CacheLayer::new());
    let engine = FluxEngine::new(cache);
    
    let manifest = Manifest {
        project_name: "integration_test".to_string(),
        dependencies: vec![
            (1, 0b0000),
            (2, 0b0001),
            (3, 0b0010),
        ],
    };
    
    // First resolve should compute
    let result1 = engine.resolve(manifest.clone()).await.unwrap();
    assert!(result1.contains("LOCK"));
    
    // Second resolve should hit cache
    let result2 = engine.resolve(manifest).await.unwrap();
    assert_eq!(result1, result2);
}

#[tokio::test]
async fn test_large_graph() {
    let cache = std::sync::Arc::new(CacheLayer::new());
    let engine = FluxEngine::new(cache);
    
    // Create a larger dependency graph
    let mut deps = Vec::new();
    for i in 0..100 {
        deps.push((i, i % 16)); // Simple conflict pattern
    }
    
    let manifest = Manifest {
        project_name: "large_graph_test".to_string(),
        dependencies: deps,
    };
    
    let result = engine.resolve(manifest).await.unwrap();
    assert!(result.contains("100"));
}
