use graphics_from_scratch::pixel_renderer::PixelRenderer;
use graphics_from_scratch::{draw_line, draw_point_cube};
use sdl2::pixels::Color;

fn rendering_benchmarks(c: &mut criterion::Criterion) {
    let width = 1000;
    let height = 1000;
    let mut pixel_renderer = PixelRenderer::new_for_benchmark(width, height);

    ////////////////////////////////////////////
    // Pixel Renderer benchmarks
    ////////////////////////////////////////////
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

    ////////////////////////////////////////////
    // Point cube benchmarks
    ////////////////////////////////////////////
    let mut bench_group = c.benchmark_group("point_cube");

    bench_group.bench_function("draw_point_cube", |b| {
        b.iter(|| {
            draw_point_cube(&mut pixel_renderer);
        })
    });

    bench_group.bench_function("draw_line", |b| {
        b.iter(|| {
            draw_line(&mut pixel_renderer, 100.0, 200.0, 300.0, 400.0);
        })
    });

    bench_group.finish();
}

criterion::criterion_group!(benches, rendering_benchmarks);
criterion::criterion_main!(benches);
