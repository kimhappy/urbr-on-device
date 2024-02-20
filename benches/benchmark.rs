use divan::{ Bencher, black_box };
use urbr::{ Layer, LSTM, Dense };

const DIM: usize =   2;
const HID: usize = 100;
const LB : usize =  10;

fn main() {
    divan::main();
}

#[divan::bench(sample_count = 256, sample_size = 256)]
fn front(bencher: Bencher) {
    let mut prev = Vec               ::new();
    let mut lstm = LSTM::< DIM, HID >::new();

    prev.extend(std::iter::repeat([0.0; DIM]).take(LB));

    bencher.bench_local(move || {
        for al in &prev {
            black_box(&mut lstm).forward(black_box(al));
        }

        black_box(black_box(&lstm).out().iter().sum::< f32 >());
    });
}

#[divan::bench(sample_count = 256, sample_size = 256)]
fn back(bencher: Bencher) {
    let mut prev  = Vec                ::new();
    let mut lstm  = LSTM ::< DIM, HID >::new();
    let mut dense = Dense::< HID, DIM >::new();

    prev.extend(std::iter::repeat([0.0; DIM]).take(LB));

    for al in &prev {
        lstm.forward(al);
    }

    bencher.bench_local(move || {
        black_box(&mut dense).forward(black_box(&lstm).out());
        black_box(black_box(&dense).out().iter().sum::< f32 >());
    });
}

#[divan::bench(sample_count = 256, sample_size = 256)]
fn full(bencher: Bencher) {
    let mut prev  = Vec                ::new();
    let mut lstm  = LSTM ::< DIM, HID >::new();
    let mut dense = Dense::< HID, DIM >::new();

    prev.extend(std::iter::repeat([0.0; DIM]).take(LB));

    bencher.bench_local(move || {
        for al in &prev {
            black_box(&mut lstm).forward(black_box(al));
        }

        black_box(&mut dense).forward(black_box(&lstm).out());
        black_box(black_box(&dense).out().iter().sum::< f32 >());
    });
}
