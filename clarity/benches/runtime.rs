use clarity::vm::execute;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fold_list(c: &mut Criterion) {
    // TODO: setup initial state
    // TODO: load transactions (vm calls)
    // TODO: time call executions
    let fold_list = "(define-private (multiply-all (x int) (acc int)) (* x acc))
         (fold multiply-all (list 1 2 3 4) 1)";

    c.bench_function("simple_folds_list", |b| {
        b.iter(|| execute(black_box(fold_list)))
    });
}

fn fold_strings(c: &mut Criterion) {
    let fold_strings =
        ["(define-private (get-len (x (string-ascii 1)) (acc int)) (+ acc 1))
         (fold get-len \"blockstack\" 0)",
        "(define-private (get-slice (x (string-ascii 1)) (acc (tuple (limit uint) (cursor uint) (data (string-ascii 10)))))
            (if (< (get cursor acc) (get limit acc))
                (let ((data (default-to (get data acc) (as-max-len? (concat (get data acc) x) u10))))
                    (tuple (limit (get limit acc)) (cursor (+ u1 (get cursor acc))) (data data))) 
                acc))
        (get data (fold get-slice \"0123456789\" (tuple (limit u5) (cursor u0) (data \"\"))))"];

    let mut g = c.benchmark_group("fold_strings");
    for i in 0..fold_strings.len() {
        g.bench_function(format!("{i}"), |b| {
            b.iter(|| execute(black_box(fold_strings[i])));
        });
    }
    g.finish();
}

criterion_group!(runtime, fold_list, fold_strings);
criterion_main!(runtime);
