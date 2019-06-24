#[derive(Clone)]
struct Mail {
    from: String,
}

fn new_mail() -> Mail {
    Mail {
        from: "eki@eqbalq.com".to_string(),
    }
}

fn send_mail(mail: &Mail) {
    fib(40);
    println!("{}", mail.from)
}

fn fib(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

use std::thread;

fn main() -> thread::Result<()> {
    let mail = new_mail();
    let mut threads = vec![];
    for i in 0..10 {
        let mut mail = mail.clone();
        let handler = thread::spawn(move || {
            mail.from = format!("corin+{}@phiware.com.au", i);
            send_mail(&mail);
        });
        threads.push(handler);
    }
    for t in threads {
        t.join()?;
    }
    Ok(())
}
