extern mod std;
extern mod mecab;

use std::arc;

fn main() {
    let s = "これはテストです";
    let model = mecab::model_new2("").unwrap();
    let model = ~arc::ARC(model);

    for 2.times {
        let model = ~arc::clone(model);

        do task::spawn {
            let model = arc::get(model);
            let tagger = model.create_tagger().unwrap();
            let lattice = model.create_lattice().unwrap();

            lattice.set_sentence(s);

            if tagger.parse_lattice(&lattice) {
                io::println("result: ");
                io::println(fmt!("%s", lattice.to_str()));
            }
        }
    }

    let model = arc::get(model);
    let tagger = model.create_tagger().unwrap();
    let lattice = model.create_lattice().unwrap();

    lattice.set_sentence(s);

    if tagger.parse_lattice(&lattice) {
        io::println("result: ");
        io::println(fmt!("%s", lattice.to_str()));
    }
}
