# 🦀 Rust Todo List — Proyek Belajar Rust untuk Pemula

Proyek mini CLI sederhana untuk mempelajari dasar-dasar bahasa pemrograman Rust
melalui aplikasi Todo List yang berjalan di terminal.

---

## Fitur

| Fitur | Keterangan |
|---|---|
| **Add Task** | Tambah task baru dengan judul |
| **List Tasks** | Tampilkan semua task beserta status (selesai / belum) |
| **Complete Task** | Tandai task sebagai selesai berdasarkan ID |
| **Delete Task** | Hapus task dari daftar berdasarkan ID |
| **Input validation** | Program tidak crash saat input tidak valid |
| **Unit tests** | 6 test bawaan menggunakan framework testing Rust |

---

## Cara Menjalankan

### Prasyarat

Pastikan Rust sudah terinstall:

```bash
cargo --version
```

Jika belum ada, install dari [https://rustup.rs](https://rustup.rs).

### Jalankan Aplikasi

```bash
cd rust-todo
cargo run
```

### Jalankan Tests

```bash
cargo test
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

Setelah `cargo run`, kamu akan melihat menu seperti ini:

```
==============================
   Rust Todo List  🦀
==============================

------------------------------
 MENU
------------------------------
 1. Add Task
 2. List Tasks
 3. Complete Task
 4. Delete Task
 5. Exit
------------------------------
Pilih menu (1-5):
```

**Contoh alur penggunaan:**

1. Ketik `1` → tekan Enter → ketik judul task → tekan Enter
2. Ketik `2` → lihat semua task dengan ID dan status
3. Ketik `3` → masukkan ID task yang ingin diselesaikan
4. Ketik `4` → masukkan ID task yang ingin dihapus
5. Ketik `5` → keluar dari program

---

## Struktur Project

```
rust-todo/
├── Cargo.toml       # Konfigurasi project dan dependencies
├── README.md        # Dokumentasi ini
└── src/
    └── main.rs      # Seluruh kode program
```

---

## Konsep Rust yang Dipelajari

### 1. `struct` — Tipe Data Kustom
```rust
struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}
```
`struct` mengelompokkan beberapa field menjadi satu tipe. Mirip class di bahasa lain,
tapi tanpa inheritance.

---

### 2. `enum` — Tipe dengan Beberapa Kemungkinan Nilai
```rust
enum TaskStatus {
    Pending,
    Completed,
}
```
`enum` memastikan status task hanya bisa bernilai `Pending` atau `Completed`,
bukan string sembarang. Compiler akan memperingatkan jika ada varian yang lupa ditangani.

---

### 3. `Vec<T>` — Array Dinamis
```rust
let mut tasks: Vec<Task> = Vec::new();
tasks.push(task);      // tambah elemen
tasks.remove(index);   // hapus berdasarkan indeks
tasks.len();           // jumlah elemen
```
`Vec` seperti `ArrayList` di Java atau `list` di Python — ukurannya bisa bertambah
saat runtime.

---

### 4. `match` — Pattern Matching
```rust
match choice.trim() {
    "1" => add_task(...),
    "2" => list_tasks(...),
    _ => println!("Tidak valid"),
}
```
`match` lebih aman daripada `if/else if` berantai karena compiler memastikan
semua kemungkinan ditangani. `_` adalah wildcard yang menangkap nilai lain.

---

### 5. `impl` — Method untuk Struct/Enum
```rust
impl Task {
    fn new(id: u32, title: String) -> Task { ... }
    fn display(&self) { ... }
}
```
`impl` menambahkan fungsi ke tipe kita. `&self` artinya method meminjam struct
tanpa mengambil ownership-nya.

---

### 6. Ownership & References (`&` dan `&mut`)
```rust
fn list_tasks(tasks: &Vec<Task>)          // pinjam read-only
fn add_task(tasks: &mut Vec<Task>, ...)   // pinjam untuk diubah
```
Ini adalah konsep unik Rust. Alih-alih menyalin data atau pakai garbage collector,
Rust meminjam data dengan aturan ketat:
- `&T` — bisa dibaca, tidak bisa diubah
- `&mut T` — bisa dibaca dan diubah, tapi hanya satu peminjam mutable pada satu waktu

---

### 7. `String` vs `&str`
```rust
let owned: String = "teks".to_string();   // String — dimiliki, ada di heap
fn prompt(s: &str) { ... }                // &str  — referensi ke teks, tidak dimiliki
```
`String` adalah teks yang bisa dimodifikasi dan dimiliki. `&str` adalah pinjaman
ke teks yang sudah ada. Gunakan `&str` untuk parameter fungsi yang hanya membaca.

---

### 8. `Result` dan Penanganan Error
```rust
let id: u32 = match input.trim().parse() {
    Ok(n)  => n,
    Err(_) => { println!("Bukan angka!"); return; }
};
```
`parse()` mengembalikan `Result<u32, _>`. Dengan `match`, kita menangani sukses
(`Ok`) dan gagal (`Err`) secara eksplisit — tidak ada pengecualian tersembunyi seperti
di Python/Java.

---

### 9. `Option` — Nilai yang Mungkin Tidak Ada
```rust
let pos: Option<usize> = tasks.iter().position(|t| t.id == id);
match pos {
    Some(index) => tasks.remove(index),
    None        => println!("Tidak ditemukan"),
}
```
`Option` menggantikan `null`. Compiler memaksa kita menangani kasus "tidak ada nilai"
sehingga tidak ada NullPointerException.

---

### 10. Unit Testing Bawaan
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_baru_berstatus_pending() {
        let task = Task::new(1, "Belajar Rust".to_string());
        assert!(matches!(task.status, TaskStatus::Pending));
    }
}
```
Rust punya testing framework bawaan. Cukup tulis fungsi bertanda `#[test]` dan
jalankan `cargo test`. Blok `#[cfg(test)]` memastikan kode test tidak ikut
dikompilasi saat build production.

---

## Langkah Selanjutnya

Setelah memahami proyek ini, kamu bisa coba:

1. **Simpan ke file** — gunakan `std::fs::write` dan `std::fs::read_to_string`
2. **Tambah due date** — tambah field `due_date: Option<String>` ke `Task`
3. **Filter task** — tampilkan hanya task Pending atau hanya Completed
4. **Prioritas** — tambah enum `Priority { Low, Medium, High }`
5. **Serialisasi** — pelajari crate `serde` + `serde_json` untuk simpan ke JSON

---

## Lisensi

MIT — bebas digunakan untuk belajar.
