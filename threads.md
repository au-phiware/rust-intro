










ruby multithreaded primer















```ruby file=first.rb
class Mailer

  def self.deliver(&block)
    mail = MailBuilder.new(&block).mail
    mail.send_mail
  end

  Mail = Struct.new(:from, :to, :subject, :body) do 
    def send_mail
      fib(30)
      puts "Email from: #{from}"
      puts "Email to  : #{to}"
      puts "Subject   : #{subject}"
      puts "Body      : #{body}"
    end

    def fib(n)
      n < 2 ? n : fib(n-1) + fib(n-2)
    end  
  end

  class MailBuilder
    def initialize(&block)
      @mail = Mail.new
      instance_eval(&block)
    end
    
    attr_reader :mail

    %w(from to subject body).each do |m|
      define_method(m) do |val|
        @mail.send("#{m}=", val)
      end
    end
  end
end

Mailer.deliver do 
  from    "eki@eqbalq.com"
  to      "jill@example.com"
  subject "Threading and Forking"
  body    "Some content"
end
```










$ ruby first.rb
Email from: eki@eqbalq.com
Email to  : jill@example.com
Subject   : Threading and Forking
Body      : Some content












```ruby file=second.rb
class Mailer
  def self.new_mail()
    mb = MailBuilder.new do 
      from    "eki@eqbalq.com"
      to      "jill@example.com"
      subject "Threading and Forking"
      body    "Some content"
    end
    mb.mail
  end

  def self.deliver(mail)
    mail.send_mail
  end

  Mail = Struct.new(:from, :to, :subject, :body) do 
    def send_mail
      fib(35)
      puts "Email from: #{from}"
    end

    def fib(n)
      n < 2 ? n : fib(n-1) + fib(n-2)
    end  
  end

  class MailBuilder
    def initialize(&block)
      @mail = Mail.new
      instance_eval(&block)
    end
    
    attr_reader :mail

    %w(from to subject body).each do |m|
      define_method(m) do |val|
        @mail.send("#{m}=", val)
      end
    end
  end
end

mail = Mailer.new_mail
mail.from = "corin@phiware.com.au"
Mailer.deliver mail
```










$ ruby second.rb
Email from: corin@phiware.com.au















```ruby file=third.rb
class Mailer
  def self.new_mail()
    mb = MailBuilder.new do 
      from    "eki@eqbalq.com"
      to      "jill@example.com"
      subject "Threading and Forking"
      body    "Some content"
    end
    mb.mail
  end

  def self.deliver(mail)
    mail.send_mail
  end

  Mail = Struct.new(:from, :to, :subject, :body) do 
    def send_mail
      fib(35)
      puts "Email from: #{from}"
    end

    def fib(n)
      n < 2 ? n : fib(n-1) + fib(n-2)
    end  
  end

  class MailBuilder
    def initialize(&block)
      @mail = Mail.new
      instance_eval(&block)
    end
    
    attr_reader :mail

    %w(from to subject body).each do |m|
      define_method(m) do |val|
        @mail.send("#{m}=", val)
      end
    end
  end
end

mail = Mailer.new_mail
threads = []
10.times do |i|
  threads << Thread.new do
    mail.from = "corin+#{i}@phiware.com.au"
    Mailer.deliver mail
  end
end
threads.map(&:join)
```










$ ruby third.rb
Email from: corin+7@phiware.com.au
Email from: corin+7@phiware.com.au
Email from: corin+7@phiware.com.au
Email from: corin+7@phiware.com.au
Email from: corin+7@phiware.com.au
Email from: corin+7@phiware.com.au
Email from: corin+7@phiware.com.au
Email from: corin+7@phiware.com.au
Email from: corin+7@phiware.com.au
Email from: corin+7@phiware.com.au













$ cargo new demo
     Created binary (application) `demo` package
$ cd demo
$ $EDITOR src/main.rs













```rust file=src/main.rs
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

fn main() {
    let mail = new_mail();
    mail.from = "corin.lawson@gmail.com".to_string();
    mail.send_mail();
}
```








error[E0594]: cannot assign to `mail.from`, as `mail` is not declared as mutable
  --> src/main.rs:27:5
   |
23 |     let mail = new_mail();
   |         ---- help: consider changing this to be mutable: `mut mail`
24 |     mail.from = "corin.lawson@gmail.com".to_string();
   |     ^^^^^^^^^ cannot assign

error: aborting due to previous error















error[E0382]: assign to part of moved value: `mail`
  --> src/main.rs:26:9
   |
24 |     let mut mail = new_mail();
   |         -------- move occurs because `mail` has type `Mail`, which does not
implement the `Copy` trait
25 |     for i in 0..10 {
26 |         mail.from = format!("corin.lawson+{}@gmail.com", i);
   |         ^^^^^^^^^ value partially assigned here after move
27 |         send_mail(mail);
   |                   ---- value moved here, in previous iteration of loop

error: aborting due to previous error












error[E0382]: assign to part of moved value: `mail`
  --> src/main.rs:26:9
   |
24 |     let mut mail = new_mail();
   |         -------- move occurs because `mail` has type `Mail`, which does not
implement the `Copy` trait
25 |     for i in 0..10 {
26 |         mail.from = format!("corin.lawson+{}@gmail.com", i);
   |         ^^^^^^^^^ value partially assigned here after move
27 |         send_mail(mail);
   |                   ---- value moved here, in previous iteration of loop

error: aborting due to previous error











error[E0308]: mismatched types
  --> src/main.rs:27:19
   |
27 |         send_mail(mail);
   |                   ^^^^
   |                   |
   |                   expected &Mail, found struct `Mail`
   |                   help: consider borrowing here: `&mail`
   |
   = note: expected type `&Mail`
              found type `Mail`

error: aborting due to previous error






