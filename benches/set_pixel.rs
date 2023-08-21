use graphics_from_scratch::pixel_renderer::PixelRenderer;
use sdl2::pixels::Color;

pub fn draw_grid(pixel_renderer: &mut PixelRenderer) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for x in 0..pixel_renderer.width {
        for y in 0..pixel_renderer.height {
            assert!(x < pixel_renderer.width);
            assert!(y < pixel_renderer.height);
            if x % 10 == 0 || y % 10 == 0 {
                pixel_renderer.set_pixel(x, y, Color::RGB(100, 100, 0));
            } else if x % 10 == 5 && y % 10 == 5 {
                pixel_renderer.set_pixel(
                    x,
                    y,
                    Color::RGB(
                        rng.gen_range(0..255),
                        rng.gen_range(0..255),
                        rng.gen_range(0..255),
                    ),
                );
            } else {
                pixel_renderer.set_pixel(x, y, Color::RGB(0, 0, 0));
            }
        }
    }
}

fn set_pixel_bench(c: &mut criterion::Criterion) {
    let width = 3440 / 2;
    let height = 1440 / 2;
    let mut pixel_renderer = PixelRenderer::new(width, height);

    let mut bench_group = c.benchmark_group("set_pixel");

    bench_group.bench_function("set_pixel", |b| b.iter(|| draw_grid(&mut pixel_renderer)));

    bench_group.finish();
}

criterion::criterion_group!(benches, set_pixel_bench);
criterion::criterion_main!(benches);
