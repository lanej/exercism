/// What should the type of _function be?
pub fn map<X, Y, F: (FnMut(X) -> Y)>(input: Vec<X>, mut function: F) -> Vec<Y> {
    let mut results = Vec::with_capacity(input.len());

    for e in input {
        results.push(function(e));
    }

    return results;
}
