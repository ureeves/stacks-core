use clarity::vm::execute;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn single_call(c: &mut Criterion) {
    // TODO: setup initial state
    // TODO: load transactions (vm calls)
    // TODO: time call executions
    let test1 = "(define-private (multiply-all (x int) (acc int)) (* x acc))
         (fold multiply-all (list 1 2 3 4) 1)";

    c.bench_function("simple_folds_list", |b| {
        b.iter(|| execute(black_box(test1)))
    });
}

criterion_group!(runtime, single_call);
criterion_main!(runtime);
