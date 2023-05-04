use std::collections::VecDeque;

pub fn predict_party_victory(senate: String) -> String {
    let mut queue: VecDeque<char> = VecDeque::new();

    let mut rr = 0;
    let mut dd = 0;
    for x in senate.chars() {
        queue.push_back(x);
        if x == 'R' { rr += 1; } else { dd += 1; }
    }

    let mut r = 0;
    let mut d = 0;
    while rr > 0 && dd > 0 {
        let cur = queue.pop_front().unwrap();
        if cur == 'R' {
            if d > 0 {
                rr -= 1;
                d -= 1;
                continue;
            } else {
                r += 1;
                queue.push_back(cur);
            }
        } else {
            if r > 0 {
                dd -= 1;
                r -= 1;
                continue;
            } else {
                d += 1;
                queue.push_back(cur);
            }
        }
    }

    if rr > 0 { return String::from("Radiant"); } else { return String::from("Dire"); };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = predict_party_victory(String::from("RD"));

        assert_eq!(result, "Radiant");
    }
}