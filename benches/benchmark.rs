use copper::enums::{Instruction, Label};
use copper::program::Program;
use criterion::{black_box, criterion_group, criterion_main, Criterion}; // Remplacez `your_crate_name` par le nom de votre crate

fn benchmark_find_label(c: &mut Criterion) {
    let mut program = Program::new();
    // Ajoutez des instructions et des étiquettes à votre programme
    for i in 0..25 {
        // random add either a label or a random instruction
        if i % 2 == 0 {
            program.add_instruction(Instruction::HLT);
        } else if i % 3 == 0 {
            program.add_instruction(Instruction::JMP(Label { name: i }));
        } else {
            program.add_instruction(Instruction::LABEL(Label { name: i }));
        }
    }

    c.bench_function("new_find_label", |b| {
        b.iter(|| {
            program.find_label(black_box(&Label { name: 5 }));
        })
    });
}

criterion_group!(benches, benchmark_find_label);
criterion_main!(benches);
