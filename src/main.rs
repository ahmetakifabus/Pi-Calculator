use ibig::UBig;
use std::time::Instant;
use std::io::{self, Write};
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, Clear, ClearType};
use crossterm::execute;

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

fn yukleme_bari_goster(ilerleme: usize, toplam: usize) {
    let bar_genisligi = 30;
    let oran = ilerleme as f64 / toplam as f64;
    let dolan_kisim = (oran * bar_genisligi as f64).round() as usize;

    let mut bar = String::new();
    for i in 0..bar_genisligi {
        if i < dolan_kisim { bar.push('='); } else { bar.push(' '); }
    }
    print!("\r    | Hesaplama Durumu: [{}] %{:.1} ", bar, oran * 100.0);
    io::stdout().flush().unwrap();
}

fn ekrani_temizle() {
    execute!(io::stdout(), Clear(ClearType::All)).unwrap();
    print!("{}", crossterm::cursor::MoveTo(0, 0));
}

fn main() {
    loop {
        ekrani_temizle();
        
        println!("██████╗ ██╗     ██████╗ █████╗ ██╗      ██████╗██╗   ██╗██╗      █████╗ ████████╗ ██████╗ ██████╗     ███╗   ███╗██╗███╗   ██╗");
        println!("██╔══██╗██║    ██╔════╝██╔══██╗██║     ██╔════╝██║   ██║██║     ██╔══██╗╚══██╔══╝██╔═══██╗██╔══██╗    ████╗ ████║██║████╗  ██║");
        println!("██████╔╝██║    ██║     ███████║██║     ██║     ██║   ██║██║     ███████║   ██║   ██║   ██║██████╔╝    ██╔████╔██║██║██╔██╗ ██║");
        println!("██╔═══╝ ██║    ██║     ██╔══██║██║     ██║     ██║   ██║██║     ██╔══██║   ██║   ██║   ██║██╔══██╗    ██║╚██╔╝██║██║██║╚██╗██║");
        println!("██║     ██║    ╚██████╗██║  ██║███████╗╚██████╗╚██████╔╝███████╗██║  ██║   ██║   ╚██████╔╝██║  ██║    ██║ ╚═╝ ██║██║██║ ╚████║");
        println!("╚═╝     ╚═╝     ╚═════╝╚═╝  ╚═╝╚══════╝ ╚═════╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝   ╚═╝    ╚═════╝ ╚═╝  ╚═╝    ╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝");                                                                                   
        print!(" -> Hesaplamak istediğiniz basamak sayısını girin: ");
        io::stdout().flush().unwrap();
        
        let mut girdi = String::new();
        io::stdin().read_line(&mut girdi).expect("Girdi okunamadı");
        
        let basamak_sayisi: u32 = match girdi.trim().parse() {
            Ok(sayi) => sayi,
            Err(_) => {
                println!("| Hata: Geçerli bir sayı girmediniz! Varsayilan: 100.000");
                100_000
            }
        };

        println!("+-------------------------------------------------------+");
        println!("    Hesaplama başlatıldı...");
        let baslangic = Instant::now();

        let c = UBig::from(426880u32);
        let mut l = UBig::from(13591409u32);
        let mut x = UBig::from(1u32);
        let mut m = UBig::from(1u32);
        let mut s = UBig::from(13591409u32);
        
        let adim_sayisi = (basamak_sayisi / 14) + 1;
        let olcek = UBig::from(10u32).pow((basamak_sayisi + 20).try_into().unwrap());

        for q in 1..=adim_sayisi {
            let q_ub = UBig::from(q);
            let k = 12 * q;
            let pay = UBig::from((k - 5) * (k - 9) * (k - 1));
            m = (m * pay) / (&q_ub * &q_ub * &q_ub);
            l += 545140134u32;
            x *= 262537412640768000u64;

            let terim = (&m * &l) / &x;
            if q % 2 == 1 { s -= terim; } else { s += terim; }

            if q % 50 == 0 || q == adim_sayisi {
                yukleme_bari_goster(q as usize, adim_sayisi as usize);
            }
        }
        println!();

        let ic_kisim = UBig::from(10005u32) * &olcek * &olcek;
        let karekok = tam_sayi_karekok(&ic_kisim);
        let mut pi = (c * karekok) / s;
        pi /= UBig::from(10u32).pow(20);

        let gecen_sure = baslangic.elapsed();
        
        let dosya_adi = format!("pi_{}.txt", basamak_sayisi);
        let pi_str = pi.to_string();
        let formatli_pi = format!("{}.{}", &pi_str[0..1], &pi_str[1..]);
        std::fs::write(&dosya_adi, formatli_pi).expect("Dosya yazılamadı");

        println!("+-------------------------------------------------------+");
        println!("|               HESAPLAMA BAŞARIYLA BİTTİ               |");
        println!("+-------------------------------------------------------+");
        println!("  -> Geçen Süre      : {:?}", gecen_sure);
        println!("  -> Hesaplanan Hane : {} basamak", basamak_sayisi);
        println!("  -> Kaydedilen Dosya: {}", dosya_adi);
        println!("+-------------------------------------------------------+");

        // --- ESC İLE ÇIKIŞ SİSTEMİ ---
        print!("\nYeni bir hesaplama yapmak için Enter'a basın (Programdan çıkmak için ESC'ye basın.)... ");
        io::stdout().flush().unwrap();

        enable_raw_mode().unwrap(); // Terminali anlık tuş okuma moduna geçiriyoruz
        let mut cikis_yapildi = false;
        
        loop {
            // Kullanıcının bastığı tuşu anında yakala
            if let Event::Key(key_event) = read().unwrap() {
                if key_event.code == KeyCode::Esc {
                    cikis_yapildi = true;
                    break;
                } else if key_event.code == KeyCode::Enter {
                    break;
                }
            }
        }
        disable_raw_mode().unwrap(); // Terminali normal moduna geri döndür

        if cikis_yapildi {
            ekrani_temizle();
            println!("Eco-Pi Calculator kapatılıyor... İyi çalışmalar!");
            break;
        }
    }
}
