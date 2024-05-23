use rug::{ops::Pow, Integer, Assign};
use std::time::Instant;

fn f(x: Integer) -> Integer {
    x.pow(2) + 1
}

fn pollard_rho(n: &Integer, x0: &Integer) -> (Integer, u64) {
    let mut x = x0.clone();
    let mut y = x0.clone();
    let mut d = Integer::new();
    let mut num_tries:u64 = 0;

    loop {
        x = f(x) % n;
        y = f(f(y) % n) % n;

        d.assign(n.gcd_ref(&Integer::from(&x - &y)));
        num_tries += 1;

        if (&1 < &d) && (&d < n) {
            break;
        }
    }
    
    (d, num_tries)
}

fn main() {
    let h = vec!["708135681371",
     "1412536792680015997",
     "95270801418092775165121913281",
     "359211301308594647469531797189639",
     "14683859981444130204708927334356982829",
     "7528575712348838132721848826467242743143",
     "52315285858560849957648945390281417094685881389288399",
     "976446009728623913234119178855528065696691676752499533",
     "577048900428714995413742965756994328276447681053651695320078699419496386017830817112881",
     "98019457094851014537689242474466616941261641321201346560044633094007486512688945300826383626603"];

    let x0 = Integer::from(2);

    let h = h.iter().map(|x| Integer::from_str_radix(x, 10).unwrap());

    println!("Starting factorization!");
    let start = Instant::now();
    for (idx, elem) in h.enumerate() {
        let result = pollard_rho(&elem, &x0);
        let q = Integer::from(&elem / &result.0);

        println!("n{} = {} * {}, antal iterationer: {}", idx + 1, result.0, q, result.1);
        debug_assert!(&(q * result.0) == &elem)
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}