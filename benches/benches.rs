use graphics_from_scratch::pixel_renderer::PixelRenderer;
use graphics_from_scratch::vec::Vec2;
use graphics_from_scratch::{draw_line, draw_mesh, get_cube_mesh, DrawOptions};
use sdl2::pixels::Color;

fn rendering_benchmarks(c: &mut criterion::Criterion) {
    let width = 1000;
    let height = 1000;
    let mut cube_mesh = get_cube_mesh();
    let mut pixel_renderer = PixelRenderer::new_for_benchmark(width, height);
    let draw_options = DrawOptions {
        draw_wireframe: true,
        fill_triangles: true,
        backface_culling: true,
        pause_rendering: true,
    };

    ////////////////////////////////////////////
    // Pixel Renderer benchmarks
    ////////////////////////////////////////////
    let mut bench_group = c.benchmark_group("PixelRenderer");

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
    let mut bench_group = c.benchmark_group("draw");

    bench_group.bench_function("draw_mesh", |b| {
        b.iter(|| {
            draw_mesh(&mut pixel_renderer, &draw_options, &mut cube_mesh);
        })
    });

    bench_group.bench_function("draw_line", |b| {
        b.iter(|| {
            draw_line(
                &mut pixel_renderer,
                Color::RGB(0, 0, 0),
                Vec2::new(100.0, 200.0),
                Vec2::new(300.0, 400.0),
            );
        })
    });

    bench_group.finish();
}

criterion::criterion_group!(benches, rendering_benchmarks);
criterion::criterion_main!(benches);
