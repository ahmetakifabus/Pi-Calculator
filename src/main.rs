use ibig::{IBig, UBig};
use rayon::prelude::*;
use std::time::Instant;
use std::io::{self, Write};
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};
use crossterm::execute;

// Babil Karekök Algoritması
fn tam_sayi_karekok(n: &UBig) -> UBig {
    if n == &UBig::from(0u32) { return UBig::from(0u32); }
    let mut x = n / UBig::from(2u32);
    if x == UBig::from(0u32) { x = UBig::from(1u32); }
    let mut y = (n / &x + &x) / UBig::from(2u32);
    while y < x {
        x = y;
        y = (n / &x + &x) / UBig::from(2u32);
    }
    x
}

fn ekrani_temizle() {
    let _ = execute!(io::stdout(), Clear(ClearType::All));
}

// İkili Bölme (Binary Splitting) yapısı
struct SplitResult {
    p: IBig,
    q: IBig,
    t: IBig,
}

// Rekürsif (Öz Yinelemeli) ve Çoklu Çekirdekli Ağaç Algoritması
fn binary_split(a: u32, b: u32) -> SplitResult {
    if b - a == 1 {
        if a == 0 {
            return SplitResult {
                p: IBig::from(1),
                q: IBig::from(1),
                t: IBig::from(13591409),
            };
        }
        
        let p_val = IBig::from(-((6 * (a as i64) - 5) * (2 * (a as i64) - 1) * (6 * (a as i64) - 1)));
        let a_i64 = IBig::from(a as i64);
        let q_val = IBig::from(10939058860032000i64) * &a_i64 * &a_i64 * &a_i64;
        let t_val = &p_val * (IBig::from(13591409i64) + IBig::from(545140134i64) * a_i64);
        
        SplitResult { p: p_val, q: q_val, t: t_val }
    } else {
        let m = (a + b) / 2;
        
        // İşlemcinin çekirdeklerine göre işi paralel olarak 2'ye böl (RAYON MAGIC)
        let (left, right) = rayon::join(|| binary_split(a, m), || binary_split(m, b));
        
        let p_val = &left.p * &right.p;
        let q_val = &left.q * &right.q;
        let t_val = &left.t * &right.q + &left.p * &right.t;
        
        SplitResult { p: p_val, q: q_val, t: t_val }
    }
}

fn main() {
    loop {
        ekrani_temizle();
        println!("██████╗ ██╗     ██████╗ █████╗ ██╗      ██████╗██╗   ██╗██╗      █████╗ ████████╗ ██████╗ ██████╗     ███╗   ███╗ █████╗ ██╗  ██╗");
        println!("██╔══██╗██║    ██╔════╝██╔══██╗██║     ██╔════╝██║   ██║██║     ██╔══██╗╚══██╔══╝██╔═══██╗██╔══██╗    ████╗ ████║██╔══██╗╚██╗██╔╝");
        println!("██████╔╝██║    ██║     ███████║██║     ██║     ██║   ██║██║     ███████║   ██║   ██║   ██║██████╔╝    ██╔████╔██║███████║ ╚███╔╝ ");
        println!("██╔═══╝ ██║    ██║     ██╔══██║██║     ██║     ██║   ██║██║     ██╔══██║   ██║   ██║   ██║██╔══██╗    ██║╚██╔╝██║██╔══██║ ██╔██╗ ");
        println!("██║     ██║    ╚██████╗██║  ██║███████╗╚██████╗╚██████╔╝███████╗██║  ██║   ██║   ╚██████╔╝██║  ██║    ██║ ╚═╝ ██║██║  ██║██╔╝ ██╗");
        println!("╚═╝     ╚═╝     ╚═════╝╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝    ╚═╝     ╚═╝╚═╝  ╚═╝╚═╝  ╚═╝");
        print!(" -> Hesaplamak istediginiz basamak sayisini girin: ");
        io::stdout().flush().unwrap();
        
        let mut girdi = String::new();
        io::stdin().read_line(&mut girdi).expect("Girdi okunamadi");
        
        let basamak_sayisi: u32 = match girdi.trim().parse() {
            Ok(sayi) => sayi,
            Err(_) => {
                println!("| Hata: Gecerli bir sayi girmediniz! Varsayilan: 100.000");
                100_000
            }
        };

        println!("\n[!] DIKKAT: Cekirdekler atese veriliyor, lutfen bekleyin...");
        let baslangic = Instant::now();

        let adim_sayisi = (basamak_sayisi / 14) + 1;

        // 1. AŞAMA: Ağaç yapısında paralel ikili bölme
        let result = binary_split(0, adim_sayisi);

        let q_ub = UBig::try_from(result.q).unwrap();
        let t_ub = UBig::try_from(result.t).unwrap();

        // 2. AŞAMA: Ölçekleme ve Karekök
        let olcek = UBig::from(10u32).pow((basamak_sayisi + 20) as usize);
        let ic_kisim = UBig::from(10005u32) * &olcek * &olcek;
        let karekok = tam_sayi_karekok(&ic_kisim);

        // 3. AŞAMA: Nihai Pi Birleştirmesi
        let c = UBig::from(426880u32);
        let pay = c * karekok * q_ub;
        let mut pi = pay / t_ub;
        
        pi /= UBig::from(10u32).pow(20);

        let gecen_sure = baslangic.elapsed();
        
        let dosya_adi = format!("pi_{}.txt", basamak_sayisi);
        let pi_str = pi.to_string();
        let formatli_pi = format!("{}.{}", &pi_str[0..1], &pi_str[1..]);
        std::fs::write(&dosya_adi, formatli_pi).expect("Dosya yazilamadi");

        println!("\n================== HESAPLAMA TAMAMLANDI =======================");
        println!("  -> Toplam Sure     : {:?}", gecen_sure);
        println!("  -> Basamak         : {}", basamak_sayisi);
        println!("  -> Kayit Dosyasi   : {}", dosya_adi);
        println!("===============================================================");

        print!("\nYeni hesaplama icin Enter, cikmak icin ESC... ");
        io::stdout().flush().unwrap();

        enable_raw_mode().unwrap();
        let mut cikis_yapildi = false;
        loop {
            if let Event::Key(key_event) = read().unwrap() {
                if key_event.code == KeyCode::Esc {
                    cikis_yapildi = true;
                    break;
                } else if key_event.code == KeyCode::Enter {
                    break;
                }
            }
        }
        disable_raw_mode().unwrap();

        if cikis_yapildi {
            ekrani_temizle();
            break;
        }
    }
}
