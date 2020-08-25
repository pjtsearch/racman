use alpm::Question;

pub fn questioncb(question: &Question) {
    println!("question {:?}", question);
    match question {
        Question::Conflict(x) => {
            let c = x.conflict();
            println!("CONFLICT BETWEEN {} AND {}", c.package1(), c.package2(),);
            println!("conflict: {}", c.reason());
        }
        _ => (),
    }
}