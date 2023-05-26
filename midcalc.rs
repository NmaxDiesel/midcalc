use std::io;

fn main() {
    let mut angka1 = String::new();
    let mut angka2 = String::new();
    let mut operasi = String::new();

    // Meminta input angka dan operasi dari pengguna
    println!("Masukkan angka pertama:");
    io::stdin().read_line(&mut angka1).expect("Gagal membaca baris");

    println!("Masukkan angka kedua:");
    io::stdin().read_line(&mut angka2).expect("Gagal membaca baris");

    println!("Masukkan operasi (+, -, *, /):");
    io::stdin().read_line(&mut operasi).expect("Gagal membaca baris");

    // Menghapus karakter baris baru pada input
    let angka1: f64 = angka1.trim().parse().expect("Angka tidak valid");
    let angka2: f64 = angka2.trim().parse().expect("Angka tidak valid");
    let operasi: char = operasi.trim().chars().next().expect("Operasi tidak valid");

    // Melakukan operasi matematika sesuai input pengguna
    let hasil = match operasi {
        '+' => angka1 + angka2,
        '-' => angka1 - angka2,
        '*' => angka1 * angka2,
        '/' => angka1 / angka2,
        _ => {
            println!("Operasi tidak valid");
            return;
        }
    };

    // Mencetak hasil
    println!("Hasil: {}", hasil);
}
