use worm_engine::physics::gpu::GpuContext;

#[tokio::test]
async fn test_gpu_compute() {
    let context = GpuContext::new().await;
    if let Some(ctx) = context {
        let input_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let output = ctx.execute_compute(&input_data).await;
        assert_eq!(output.len(), 6);
        assert_eq!(output[0], 2.0);
        assert_eq!(output[1], 4.0);
        assert_eq!(output[2], 6.0);
        assert_eq!(output[3], 8.0);
        assert_eq!(output[4], 10.0);
        assert_eq!(output[5], 12.0);
    } else {
        // GPU not available, skip test
    }
}
