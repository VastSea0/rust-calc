use std::io;

fn main() {
    println!("Hello, world!");
    println!("Welcome to calculator!");

    // Değişkenler String olarak başlatılıyor
    let mut number_one = String::new();
    let mut number_two = String::new();
    let mut islem = String::new();

    // İlk sayıyı kullanıcıdan alıyoruz
    println!("Please enter the first number");
    io::stdin().read_line(&mut number_one).unwrap();

    // İlk sayıyı integer'a çeviriyoruz
    let number_one: i32 = match number_one.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for number one");
            return;
        }
    };

    // İkinci sayıyı kullanıcıdan alıyoruz
    // Burada `println!` makrosu ile kullanıcıya bir mesaj veriyoruz
    // ve "Please enter the second number" mesajını konsola yazdırıyoruz.
    // Ardından `io::stdin().read_line(&mut number_two)` ile
    // kullanıcının klavyeden girdiği değeri alıyoruz.
    // `read_line` fonksiyonu, kullanıcının girdiği değeri `number_two`
    // değişkenine yerleştirir ve bu değişkenin tipi String olmalıdır.
    // `.unwrap()` ise bu işlem sırasında oluşabilecek hataları yakalamak
    // için kullanılır. Eğer bir hata olursa, program burada panikleyerek
    // hata mesajı gösterir.
    println!("Please enter the second number");
    io::stdin().read_line(&mut number_two).unwrap();


    // İkinci sayıyı integer'a çeviriyoruz
    // Kullanıcının girdiği değeri önce .trim() ile baştaki ve sondaki
    // boşluklardan arındırıyoruz. Ardından .parse() fonksiyonu ile
    // bu değeri i32 türüne çeviriyoruz. Eğer dönüşüm başarılı olursa
    // Ok(num) döner ve 'num' değişkenine atanır. Başarısız olursa
    // Err(_) döner ve "Invalid input" mesajı verip program sonlandırılır.
    let number_two: i32 = match number_two.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for number two");
            return;
        }
    };

    // İşlem türünü alıyoruz
    println!("Please enter the transaction (+, -, *, /)");
    io::stdin().read_line(&mut islem).unwrap();

    let islem = islem.trim();  // Boşlukları temizliyoruz

    //.trim() fonkisyonu boşlukları temizler
    // fonksiyonu bir String veya &str üzerindeki baştaki ve
    // sondaki boşluk karakterlerini (space, tab, newline, vb.)
    // temizler. trim() fonksiyonu, karakter dizisinin içindeki
    // boşlukları etkilemez, yalnızca başında ve sonunda
    // yer alanları kaldırır.
    // Örnek:
        // let input = "   Hello, world!   ";
        // let trimmed = input.trim();
        // println!("'{}'", trimmed);  // Çıktı: 'Hello, world!'


    println!("Number one: {}", number_one);
    println!("Number two: {}", number_two);
    println!("islem: {}", islem);

    // İşleme göre sonucu hesaplıyoruz

    match islem {
        "+" => {
            println!("+");
            let result = number_one + number_two;
            println!("Result: {}", result);
        },
        "-" => {
            println!("-");
            let result = number_one - number_two;
            println!("Result: {}", result);
        },
        "*" => {
            println!("*");
            let result = number_one * number_two;
            println!("Result: {}", result);
        },
        "/" => {
            // Bölme işleminde ikinci sayının sıfır olup olmadığını kontrol ediyoruz
            if number_two != 0 {
                println!("/");
                let result = number_one / number_two;
                println!("Result: {}", result);
            } else {
                println!("Error: Division by zero is not allowed.");
            }
        },
        _ => panic!("Invalid Transaction!"),
    }
}
