#[macro_use]
extern crate criterion;
extern crate game_of_life;

use criterion::black_box;
use criterion::Criterion;

use game_of_life::game::Game;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut g = Game::new(10, 10);

    c.bench_function("Game Iterations", |b| {
        b.iter(|| {
            for _ in 0..1000 {
                black_box(g.next())
            }
        })
    });

    let mut g = Game::new(10, 10);
    c.bench_function("Large Game Iterations", |b| {
        b.iter(|| {
            for _ in 0..1000000 {
                black_box(g.next())
            }
        })
    });

    let mut g = Game::new(100, 100);
    c.bench_function("Large Game board Iterations", |b| {
        b.iter(|| {
            for _ in 0..100 {
                black_box(g.next())
            }
        })
    });

    c.bench_function("Large Gameboard", |b| {
        b.iter(|| black_box(Game::new(100, 1000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
