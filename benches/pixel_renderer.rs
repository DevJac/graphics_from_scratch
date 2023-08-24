use graphics_from_scratch::pixel_renderer::PixelRenderer;
use sdl2::pixels::Color;

fn pixel_renderer_bench(c: &mut criterion::Criterion) {
    let width = 1000;
    let height = 1000;
    let mut pixel_renderer = PixelRenderer::new_for_benchmark(width, height);

    let mut bench_group = c.benchmark_group("pixel_renderer");

    bench_group.bench_function("set_pixel", |b| {
        b.iter(|| {
            pixel_renderer.set_pixel(500, 500, Color::RGB(50, 100, 150));
        })
    });

    bench_group.bench_function("clear_pixels", |b| {
        b.iter(|| {
            pixel_renderer.clear_pixels(Color::RGB(50, 100, 150));
        })
    });

    bench_group.bench_function("render", |b| {
        b.iter(|| {
            pixel_renderer.render();
        })
    });

    bench_group.finish();
}

criterion::criterion_group!(benches, pixel_renderer_bench);
criterion::criterion_main!(benches);
