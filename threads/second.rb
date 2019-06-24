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
