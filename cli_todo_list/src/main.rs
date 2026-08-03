// ============================================================
// rust-todo: Aplikasi CLI Todo List untuk Belajar Rust
// ============================================================
//
// Konsep Rust yang dipelajari di file ini:
//   - struct      : mendefinisikan tipe data kustom
//   - enum        : tipe yang memiliki beberapa kemungkinan nilai
//   - Vec<T>      : array dinamis (growable list)
//   - match       : pencocokan pola (pattern matching)
//   - impl        : menambahkan method ke struct / enum
//   - ownership   : String vs &str
//   - Result / unwrap_or : penanganan error ringan
//   - format!     : memformat teks seperti printf
//   - fn          : mendefinisikan fungsi
// ============================================================

use std::io::{self, Write};

// ----------------------------------------------------------
// ENUM: Status sebuah task.
//
// `enum` di Rust mendefinisikan tipe yang hanya bisa bernilai
// salah satu dari varian yang terdaftar.
//
// `#[derive(Debug)]` membuat Rust otomatis bisa mencetak
// nilai enum ini untuk keperluan debugging.
// ----------------------------------------------------------
#[derive(Debug)]
enum TaskStatus {
    Pending,
    Completed,
}

// Tambahkan method ke TaskStatus agar bisa ditampilkan sebagai teks.
impl TaskStatus {
    /// Mengembalikan label teks yang mudah dibaca.
    fn label(&self) -> &str {
        // `match` memastikan semua varian ditangani — compiler akan
        // memberi error jika ada varian yang terlewat.
        match self {
            TaskStatus::Pending => "[ ]",
            TaskStatus::Completed => "[x]",
        }
    }
}

// ----------------------------------------------------------
// STRUCT: Representasi sebuah task.
//
// `struct` mengelompokkan beberapa field terkait menjadi
// satu tipe data baru — mirip class tanpa inheritance.
// ----------------------------------------------------------
struct Task {
    id: u32,       // ID unik, bertipe u32 (unsigned 32-bit integer)
    title: String, // Judul task — String adalah teks yang dimiliki (owned)
    status: TaskStatus,
}

// Tambahkan method ke Task agar bisa mencetak baris ringkasan.
impl Task {
    /// Membuat Task baru dengan status Pending secara default.
    fn new(id: u32, title: String) -> Task {
        Task {
            id,
            title,
            status: TaskStatus::Pending,
        }
    }

    /// Menampilkan satu baris ringkasan task.
    fn display(&self) {
        // `&self` artinya method meminjam Task tanpa mengambil ownership-nya.
        println!(
            "  {}  [ID: {:>3}]  {}",
            self.status.label(),
            self.id,
            self.title
        );
    }
}

// ----------------------------------------------------------
// FUNGSI UTAMA: main() — titik masuk program Rust
// ----------------------------------------------------------
fn main() {
    // `Vec<Task>` adalah list task yang bisa tumbuh secara dinamis.
    let mut tasks: Vec<Task> = Vec::new();

    // Counter untuk ID task berikutnya.
    let mut next_id: u32 = 1;

    println!("==============================");
    println!("   Rust Todo List  🦀");
    println!("==============================");

    // Loop utama program — terus berjalan sampai pengguna memilih Exit.
    loop {
        print_menu();

        let choice = read_input("Pilih menu (1-5): ");

        // `match` di sini menangani setiap pilihan menu.
        // `trim()` menghapus spasi/newline di awal & akhir input.
        match choice.trim() {
            "1" => add_task(&mut tasks, &mut next_id),
            "2" => list_tasks(&tasks),
            "3" => complete_task(&mut tasks),
            "4" => delete_task(&mut tasks),
            "5" => {
                println!("\nSampai jumpa! 👋");
                break; // Keluar dari loop
            }
            // `_` adalah wildcard — menangkap semua nilai lain.
            _ => println!("\n⚠️  Pilihan tidak valid. Masukkan angka 1-5.\n"),
        }
    }
}

// ----------------------------------------------------------
// Mencetak menu pilihan ke layar.
// ----------------------------------------------------------
fn print_menu() {
    println!("\n------------------------------");
    println!(" MENU");
    println!("------------------------------");
    println!(" 1. Add Task");
    println!(" 2. List Tasks");
    println!(" 3. Complete Task");
    println!(" 4. Delete Task");
    println!(" 5. Exit");
    println!("------------------------------");
}

// ----------------------------------------------------------
// Membaca satu baris input dari pengguna.
//
// Parameter `prompt: &str` memakai referensi (&) ke string —
// bukan menyalin atau memiliki string tersebut.
// Fungsi mengembalikan String (owned).
// ----------------------------------------------------------
fn read_input(prompt: &str) -> String {
    // Cetak prompt tanpa newline agar kursor tetap di baris yang sama.
    print!("{}", prompt);

    // `flush()` memastikan teks prompt benar-benar tampil sebelum input.
    io::stdout().flush().unwrap_or(());

    let mut input = String::new();

    // `io::stdin().read_line()` membaca baris dari keyboard.
    // Jika gagal (misal pipe tertutup), kembalikan string kosong.
    io::stdin().read_line(&mut input).unwrap_or(0);

    input
}

// ----------------------------------------------------------
// Menambahkan task baru ke daftar.
//
// `tasks: &mut Vec<Task>` — meminjam Vec secara mutable
//   agar kita bisa menambah elemen.
// `next_id: &mut u32` — meminjam counter agar bisa diubah.
// ----------------------------------------------------------
fn add_task(tasks: &mut Vec<Task>, next_id: &mut u32) {
    let input = read_input("\nJudul task baru: ");
    let title = input.trim();

    // Validasi: judul tidak boleh kosong.
    if title.is_empty() {
        println!("⚠️  Judul task tidak boleh kosong.");
        return; // Keluar dari fungsi lebih awal
    }

    // `to_string()` mengubah &str menjadi String (owned copy).
    let task = Task::new(*next_id, title.to_string());

    println!("✅  Task '{}' ditambahkan (ID: {}).", task.title, task.id);

    tasks.push(task); // Tambahkan ke Vec
    *next_id += 1; // `*` untuk menulis ke nilai di balik referensi (dereference)
}

// ----------------------------------------------------------
// Menampilkan semua task yang ada.
//
// `tasks: &Vec<Task>` — meminjam Vec secara read-only (immutable).
// ----------------------------------------------------------
fn list_tasks(tasks: &Vec<Task>) {
    println!();

    if tasks.is_empty() {
        println!("📋  Belum ada task. Tambahkan dengan pilih menu 1.");
        return;
    }

    let total = tasks.len();
    let done = tasks
        .iter()
        .filter(|t| matches!(t.status, TaskStatus::Completed))
        .count();

    println!("📋  Daftar Task ({}/{} selesai):", done, total);
    println!("  --------------------------------");

    // `for` loop mengiterasi setiap &Task di Vec (referensi, bukan copy).
    for task in tasks {
        task.display();
    }

    println!("  --------------------------------");
}

// ----------------------------------------------------------
// Menandai sebuah task sebagai Completed.
// ----------------------------------------------------------
fn complete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("\n⚠️  Belum ada task.");
        return;
    }

    list_tasks(tasks);

    let input = read_input("Masukkan ID task yang ingin diselesaikan: ");

    // `parse::<u32>()` mengubah String ke angka u32.
    // Hasilnya adalah Result<u32, _>; kita tangani error dengan pesan jelas.
    let id: u32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("⚠️  ID tidak valid. Masukkan angka.");
            return;
        }
    };

    // `iter_mut()` memberikan referensi mutable ke setiap elemen.
    for task in tasks.iter_mut() {
        if task.id == id {
            match task.status {
                TaskStatus::Completed => {
                    println!("ℹ️  Task '{}' sudah selesai sebelumnya.", task.title);
                }
                TaskStatus::Pending => {
                    task.status = TaskStatus::Completed;
                    println!("✅  Task '{}' ditandai selesai!", task.title);
                }
            }
            return;
        }
    }

    println!("⚠️  Task dengan ID {} tidak ditemukan.", id);
}

// ----------------------------------------------------------
// Menghapus sebuah task dari daftar.
// ----------------------------------------------------------
fn delete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("\n⚠️  Belum ada task untuk dihapus.");
        return;
    }

    list_tasks(tasks);

    let input = read_input("Masukkan ID task yang ingin dihapus: ");

    let id: u32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("⚠️  ID tidak valid. Masukkan angka.");
            return;
        }
    };

    // `position()` mencari indeks elemen pertama yang cocok dengan kondisi.
    // Hasilnya Option<usize>: Some(index) jika ditemukan, None jika tidak.
    let pos = tasks.iter().position(|t| t.id == id);

    match pos {
        Some(index) => {
            // `remove(index)` menghapus elemen pada indeks tersebut
            // dan mengembalikan nilai yang dihapus (owned).
            let removed = tasks.remove(index);
            println!("🗑️  Task '{}' berhasil dihapus.", removed.title);
        }
        None => {
            println!("⚠️  Task dengan ID {} tidak ditemukan.", id);
        }
    }
}

// ----------------------------------------------------------
// TES OTOMATIS
//
// Rust punya sistem testing bawaan.
// Jalankan dengan: cargo test
//
// Blok `#[cfg(test)]` hanya dikompilasi saat `cargo test`.
// ----------------------------------------------------------
#[cfg(test)]
mod tests {
    // `super::*` mengimpor semua item dari modul parent (file ini).
    use super::*;

    #[test]
    fn test_task_baru_berstatus_pending() {
        let task = Task::new(1, "Belajar Rust".to_string());

        // Memastikan status awal adalah Pending.
        assert!(matches!(task.status, TaskStatus::Pending));
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Belajar Rust");
    }

    #[test]
    fn test_status_label_pending() {
        let status = TaskStatus::Pending;
        assert_eq!(status.label(), "[ ]");
    }

    #[test]
    fn test_status_label_completed() {
        let status = TaskStatus::Completed;
        assert_eq!(status.label(), "[x]");
    }

    #[test]
    fn test_tambah_task_ke_vec() {
        let mut tasks: Vec<Task> = Vec::new();
        let mut next_id: u32 = 1;

        // Simulasi add_task secara langsung (tanpa input keyboard).
        tasks.push(Task::new(next_id, "Task A".to_string()));
        next_id += 1;
        tasks.push(Task::new(next_id, "Task B".to_string()));

        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].id, 1);
        assert_eq!(tasks[1].id, 2);
    }

    #[test]
    fn test_complete_task_mengubah_status() {
        let mut tasks = vec![Task::new(1, "Baca buku".to_string())];

        // Ubah status task pertama menjadi Completed.
        tasks[0].status = TaskStatus::Completed;

        assert!(matches!(tasks[0].status, TaskStatus::Completed));
    }

    #[test]
    fn test_delete_task_mengurangi_jumlah() {
        let mut tasks = vec![
            Task::new(1, "Task satu".to_string()),
            Task::new(2, "Task dua".to_string()),
        ];

        // Hapus task dengan ID 1.
        let pos = tasks.iter().position(|t| t.id == 1);
        if let Some(index) = pos {
            tasks.remove(index);
        }

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].id, 2);
    }

    #[test]
    fn test_delete_id_tidak_ada() {
        let tasks = vec![Task::new(1, "Task satu".to_string())];

        let pos = tasks.iter().position(|t| t.id == 99); // ID tidak ada
        assert!(pos.is_none()); // Tidak ditemukan = None
        assert_eq!(tasks.len(), 1); // Vec tidak berubah
    }
}
