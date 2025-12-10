use core::f64;
use std::io;

// Definisi fungsi main, titik masuk program
fn main() {
    println!("=== Simple CLI Calculator ===");
    loop {
        // Cetak header kalkulator
        println!("=== Simple CLI Calculator ===");
        // Input angka pertama dengan memanggil fungsi read_number
        let number1 = read_number("Enter the first number: ");
        // Input operator dengan memanggil fungsi read_operator
        let operator = read_operator("Enter an operator (+, -, *, /): ");
        // Input angka kedua dengan memanggil fungsi read_number
        let number2 = read_number("Enter the second number: ");
        // Hitung hasil dengan memanggil fungsi calculate
        let result = calculate(number1, number2, operator);
        // Tampilkan hasil perhitungan
        println!("Result: {} {} {} = {}", number1, operator, number2, result);

        // Inisialisasi string kosong untuk menyimpan input pengguna tentang apakah ingin melanjutkan
        let mut continue_input = String::new();
        print!("Do you want to perform another calculation? (y/n): ");
        // Flush stdout agar prompt ditampilkan segera sebelum program menunggu input
        let _ = io::Write::flush(&mut io::stdout());
        // Baca baris input dari stdin dan simpan ke continue_input, gunakan unwrap untuk menangani error
        io::stdin().read_line(&mut continue_input).unwrap();
        // Jika input setelah dipotong spasi dan diubah ke lowercase bukan "y", keluar dari loop
        if continue_input.trim().to_lowercase() != "y" {
            break;
        }
    }
    println!("Thank you for using the calculator. Goodbye!");
}

// Definisi fungsi read_number yang menerima prompt sebagai string slice dan mengembalikan f64
fn read_number(prompt: &str) -> f64 {
    // Mulai loop tak terbatas untuk terus meminta input hingga valid
    loop {
        // Inisialisasi string kosong untuk menyimpan input pengguna
        let mut input = String::new();
        // Cetak prompt tanpa newline agar input langsung mengikuti prompt
        print!("{}", prompt);
        // Flush stdout agar prompt ditampilkan segera sebelum membaca input
        let _ = io::Write::flush(&mut io::stdout());
        // Baca baris input dari stdin dan simpan ke input, gunakan unwrap untuk menangani error
        io::stdin().read_line(&mut input).unwrap();
        // Cocokkan hasil parsing input yang dipotong spasi menjadi f64
        match input.trim().parse::<f64>() {
            // Jika parsing berhasil, kembalikan nilai f64
            Ok(v) => return v,
            // Jika parsing gagal, cetak pesan error dan lanjutkan loop
            Err(_) => {
                println!("Invalid number. Please try again.");
            }
        }
    }
}

// Definisi fungsi read_operator yang menerima prompt sebagai string slice dan mengembalikan char
fn read_operator(prompt: &str) -> char {
    // Mulai loop tak terbatas untuk terus meminta input hingga operator valid
    loop {
        // Inisialisasi string kosong untuk menyimpan input pengguna
        let mut input = String::new();
        // Cetak prompt tanpa newline agar input langsung mengikuti
        print!("{}", prompt);
        // Flush stdout agar prompt ditampilkan segera sebelum membaca input
        let _ = io::Write::flush(&mut io::stdout());
        // Baca baris input dari stdin dan simpan ke input, gunakan unwrap untuk menangani error
        io::stdin().read_line(&mut input).unwrap();
        // Potong spasi dari input dan simpan sebagai operator
        let operator = input.trim();
        // Periksa apakah panjang operator adalah 1 karakter
        if operator.len() == 1 {
            // Ambil karakter pertama dari operator
            let c = operator.chars().next().unwrap();
            // Periksa apakah karakter tersebut adalah salah satu dari +, -, *, /
            if "+-*/".contains(c) {
                // Jika ya, kembalikan karakter tersebut
                return c;
            }
        }
        println!("Invalid operator. Please enter one of +, -, *, /");
    }
}

// Definisi fungsi calculate yang menerima dua f64 dan satu char operator, mengembalikan f64 hasil perhitungan
fn calculate(a: f64, b: f64, operator: char) -> f64 {
    // Gunakan match untuk menentukan operasi berdasarkan operator
    match operator {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b == 0.0 {
                println!("Warning: Division by zero detected. Returning infinity.");
                f64::INFINITY
            } else {
                a / b
            }
        }
        // Untuk operator lain yang tidak dikenal, kembalikan 0.0 sebagai default
        _ => 0.00,
    }
}
