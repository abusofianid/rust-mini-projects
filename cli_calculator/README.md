# 🦀 Rust CLI Calculator — Proyek Belajar Rust untuk Pemula

Proyek mini CLI sederhana untuk mempelajari dasar-dasar bahasa pemrograman Rust
melalui aplikasi kalkulator yang berjalan di terminal.

---

## Fitur

| Fitur | Keterangan |
|---|---|
| **Operasi dasar** | Tambah (`+`), kurang (`-`), kali (`*`), dan bagi (`/`) |
| **Angka desimal** | Semua angka diproses sebagai `f64`, jadi `3.5 * 2` juga bisa |
| **Input validation** | Angka atau operator yang tidak valid akan diminta ulang, program tidak crash |
| **Pembagian dengan nol** | Menampilkan peringatan dan mengembalikan `inf` |
| **Hitung berulang** | Setelah satu perhitungan, kamu bisa lanjut dengan menjawab `y` |

---

## Cara Menjalankan

### Prasyarat

Pastikan Rust sudah terinstall (proyek ini memakai Rust edition 2024, jadi butuh Rust 1.85 atau lebih baru):

```bash
cargo --version
```

Jika belum ada, install dari [https://rustup.rs](https://rustup.rs).

### Jalankan Aplikasi

```bash
cd cli_calculator
cargo run
```

### Format Kode

```bash
cargo fmt
```

### Cek Error Tanpa Kompilasi Penuh

```bash
cargo check
```

---

## Cara Menggunakan

Setelah `cargo run`, program akan meminta dua angka dan satu operator:

```
=== Simple CLI Calculator ===
=== Simple CLI Calculator ===
Enter the first number: 10
Enter an operator (+, -, *, /): *
Enter the second number: 5
Result: 10 * 5 = 50
Do you want to perform another calculation? (y/n): n
Thank you for using the calculator. Goodbye!
```

**Contoh alur penggunaan:**

1. Masukkan angka pertama, lalu tekan Enter
2. Masukkan operator (`+`, `-`, `*`, atau `/`), lalu tekan Enter
3. Masukkan angka kedua, lalu tekan Enter
4. Hasil perhitungan langsung ditampilkan
5. Ketik `y` untuk menghitung lagi, atau tombol lain untuk keluar

**Contoh input tidak valid:**

```
Enter the first number: abc
Invalid number. Please try again.
Enter an operator (+, -, *, /): x
Invalid operator. Please enter one of +, -, *, /
```

---

## Struktur Project

```
cli_calculator/
├── Cargo.toml       # Konfigurasi project dan dependencies
├── README.md        # Dokumentasi ini
└── src/
    └── main.rs      # Seluruh kode program
```

Tidak ada dependency eksternal — proyek ini hanya memakai Rust standard library.

---

## Konsep Rust yang Dipelajari

### 1. Fungsi dengan Tipe Parameter dan Return
```rust
fn calculate(a: f64, b: f64, operator: char) -> f64 { ... }
```
Rust mewajibkan tipe parameter dan tipe return ditulis eksplisit. `f64` adalah
bilangan desimal 64-bit, sedangkan `char` adalah satu karakter.

---

### 2. `loop`, `break`, dan `return`
```rust
loop {
    // ... minta input ...
    if continue_input.trim().to_lowercase() != "y" {
        break;
    }
}
```
`loop` berjalan tanpa henti sampai ada `break` (keluar dari loop) atau `return`
(keluar dari fungsi). Di proyek ini `loop` dipakai dua kali: untuk mengulang
kalkulator, dan untuk terus meminta input sampai valid.

---

### 3. Membaca Input dari Terminal (`stdin`)
```rust
let mut input = String::new();
io::stdin().read_line(&mut input).unwrap();
```
`read_line` menulis input ke `String` yang kita pinjamkan secara mutable
(`&mut input`). Hasilnya selalu menyertakan karakter newline, jadi kita
memanggil `.trim()` sebelum memprosesnya.

---

### 4. Menampilkan Prompt dengan `flush`
```rust
print!("{}", prompt);
let _ = io::Write::flush(&mut io::stdout());
```
`print!` (tanpa `ln`) tidak mencetak newline, dan output-nya bisa tertahan di
buffer. `flush` memaksa prompt tampil sebelum program menunggu input.

---

### 5. `String` vs `&str`
```rust
fn read_number(prompt: &str) -> f64 { ... }
```
`String` adalah teks yang dimiliki dan bisa diubah. `&str` adalah pinjaman ke
teks yang sudah ada. Untuk parameter fungsi yang hanya membaca teks, gunakan `&str`.

---

### 6. `Result` dan Penanganan Error
```rust
match input.trim().parse::<f64>() {
    Ok(v)  => return v,
    Err(_) => println!("Invalid number. Please try again."),
}
```
`parse()` mengembalikan `Result`. Dengan `match`, kita menangani sukses (`Ok`)
dan gagal (`Err`) secara eksplisit, tidak ada exception tersembunyi.

---

### 7. `match` — Pattern Matching
```rust
match operator {
    '+' => a + b,
    '-' => a - b,
    '*' => a * b,
    '/' => { /* ... */ }
    _   => 0.00,
}
```
`match` memilih cabang berdasarkan nilai. `_` adalah wildcard untuk nilai lain,
dan compiler memastikan semua kemungkinan tertangani.

---

### 8. `if` sebagai Ekspresi dan `f64::INFINITY`
```rust
if b == 0.0 {
    println!("Warning: Division by zero detected. Returning infinity.");
    f64::INFINITY
} else {
    a / b
}
```
Di Rust, `if/else` menghasilkan nilai, sehingga bisa jadi hasil akhir sebuah
cabang `match`. Bilangan desimal (`f64`) punya nilai khusus `INFINITY`, jadi
pembagian dengan nol tidak membuat program crash.

---

## Langkah Selanjutnya

Setelah memahami proyek ini, kamu bisa coba:

1. **Unit test** — tambah `#[cfg(test)]` untuk menguji fungsi `calculate`
2. **Error handling yang lebih baik** — ganti `unwrap()` dengan `?` atau `match`, dan buat `calculate` mengembalikan `Result<f64, String>` untuk kasus bagi nol
3. **Operator baru** — tambah modulo (`%`) dan pangkat
4. **Riwayat perhitungan** — simpan hasil sebelumnya ke `Vec<String>` dan tampilkan di akhir
5. **Parser ekspresi** — izinkan input seperti `2 + 3 * 4` dalam satu baris
