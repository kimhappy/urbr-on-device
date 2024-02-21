use divan::{ Bencher, black_box };
use urbr::{ Layer, Front, Back, DIM, HIDDEN, LOOK_BACK };

fn main() {
    divan::main();
}

#[divan::bench(sample_count = 256, sample_size = 256)]
fn front(bencher: Bencher) {
    let mut prev = Vec  ::new();
    let mut lstm = Front::new();

    prev.extend(std::iter::repeat([0.0; DIM]).take(LOOK_BACK));

    bencher.bench_local(move || {
        for al in &prev {
            black_box(&mut lstm).forward(black_box(al));
        }

        black_box(black_box(&lstm).out().iter().sum::< f32 >());
    });
}

#[divan::bench(sample_count = 256, sample_size = 256)]
fn back(bencher: Bencher) {
    let     lstm_out = [0.0; HIDDEN];
    let mut dense    = Back::new();

    bencher.bench_local(move || {
        black_box(&mut dense).forward(black_box(&lstm_out));
        black_box(black_box(&dense).out().iter().sum::< f32 >());
    });
}

#[divan::bench(sample_count = 256, sample_size = 256)]
fn full(bencher: Bencher) {
    let mut prev  = Vec  ::new();
    let mut lstm  = Front::new();
    let mut dense = Back ::new();

    prev.extend(std::iter::repeat([0.0; DIM]).take(LOOK_BACK));

    bencher.bench_local(move || {
        for al in &prev {
            black_box(&mut lstm).forward(black_box(al));
        }

        black_box(&mut dense).forward(black_box(&lstm).out());
        black_box(black_box(&dense).out().iter().sum::< f32 >());
    });
}
