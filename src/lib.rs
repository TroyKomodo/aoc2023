use std::io::BufRead;

pub fn get_input<S: FromIterator<String>>() -> S {
    std::io::stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .collect()
}

pub fn get_input_from<'a, S: FromIterator<&'a str>>(input: &'a str) -> S {
    input.lines().collect()
}
