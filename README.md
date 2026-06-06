# Pi Calculator

Pure Rust ile geliştirilmiş, hem yüksek performanslı hem de çevre dostu (eco-friendly) terminal tabanlı bir Pi sayısı hesaplayıcı. Bu araç, harici C/C++ matematik kütüphanelerine (GMP/MPFR gibi) bağımlı kalmadan Saf Rust (Pure Rust) ekosistemi üzerinde çalışır. Bu sayede hem Windows hem de Pardus (Linux) gibi işletim sistemlerinde hiçbir bağımlılık hatası vermeden doğrudan derlenebilir.

Proje, dünyanın en hızlı yakınsayan Pi hesaplama yöntemi olan Chudnovsky Algoritması'nı ve büyük tam sayılarda muazzam hızlı çalışan Babil Yöntemi (Newton-Raphson) Karekök Algoritması'nı kullanır.

## Özellikler

### Chudnovsky Algoritması:
Döngünün sadece tek bir adımında bile virgülden sonra yaklaşık 14 doğru basamak hesaplayan, dünyanın en güçlü yakınsama algoritması.

### Saf Rust (Pure Rust) Motoru:
Donanımsal 64-bit sınırlarına takılmayan ibig kütüphanesi üzerine inşa edilmiştir. Windows ve Linux uyumluluğu için harici C bağımlılıklarından tamamen arındırılmıştır.

### Şık Terminal Arayüzü (TUI):
Nostaljik ve profesyonel CLI araçları tarzında, çizgilerle (-, +, |) ve ASCII logolarıyla tasarlanmış kullanıcı dostu arayüz.

### İşlemci Dostu Yükleme Barı (Progress Bar):
Devasa hesaplamalar yaparken işlemciyi ekstradan yormamak için her 50 adımda bir kendini güncelleyen animasyonlu durum çubuğu.

### Akıllı Kayıt Sistemi:
Hesapladığınız her basamak değerini bir öncekinin üzerine yazmaz; dinamik olarak pi_1000.txt, pi_1000000.txt gibi dosya adlarıyla klasörünüze kaydeder.

### Çapraz Platform Ekran Temizleme:
İşletim sisteminden bağımsız olarak (cls veya clear tetiklemeden) terminali bellek seviyesinde temizleyen çapraz platform mimari.

### Gelişmiş Döngü ve ESC ile Çıkış:
Program her hesaplamadan sonra otomatik olarak başa döner. Kapatmak için Enter tuşunu beklemek yerine anlık olarak ESC tuşuna basmanız yeterlidir (Terminal Ham Modu/Raw Mode destekli).

## Kurulum ve Derleme

Projenin bilgisayarınızda derlenebilmesi için sisteminizde Rust ve Cargo'nun kurulu olması gerekir.

### 1. Depoyu Klonlayın veya İndirin

git clone [https://github.com/ahmetakifabus/Pi-Calculator.git](https://github.com/ahmetakifabus/Pi-Calculator.git)

cd Pi-Calculator


### 2. Bağımlılıkları Kontrol Edin (Cargo.toml)

Projenin tıkır tıkır çalışması için Cargo.toml dosyanızın dependencies kısmı şu şekilde olmalıdır:

[dependencies]
ibig = "0.3.6"
crossterm = "0.27"


### 3. En Optimize (Eco-Friendly) Modda Çalıştırın

İşlemci çekirdeklerini %100 verimle çalıştırmak, watt başına düşen işlem miktarını maksimize etmek ve gereksiz hiçbir enerji harcatmamak için projeyi mutlaka --release bayrağıyla çalıştırın:

cargo run --release

Derlenen bağımsız çalıştırılabilir dosyaya (Binary) ulaşmak için:

Windows: target/release/pi_calculator.exe

Linux: target/release/pi_calculator

## Debian Kurulumu (.deb)

Eğer bu aracı herhangi bir Debian tabanlı Linux dağıtımına kalıcı olarak kurmak isterseniz, doğrudan bir .deb paketi üretebilirsiniz.

### 1. Debian Paketleyiciyi Kurun (Evdeki Linux/WSL Ortamında)

cargo install cargo-deb


### 2. Paketi Oluşturun

Proje klasörünün içindeyken şu komutu çalıştırın:

cargo deb

Bu komut target/debian/pi-calculator_0.1.0-1_amd64.deb dosyasını başarıyla üretecektir.

## Matematiksel Arka Plan

### Chudnovsky Formülü

Algoritma, Ramanujan-Chudnovsky serisine dayanır:

$$\frac{1}{\pi} = 12 \sum^\infty_{q=0} \frac{(-1)^q (6q)! (13591409 + 545140134q)}{(3q)!(q!)^3 (640320)^{3q + 3/2}}$$

Bu kodda, tam sayı aritmetiğinde hassasiyeti kaybetmemek adına tüm sabitler hedef basamak büyüklüğüne göre dinamik olarak ölçeklendirilir (scale).

### Babil (Newton-Raphson) Karekök Yöntemi

Formüldeki $\sqrt{10005}$ değerini kusursuz hassasiyette hesaplayabilmek için kullanılan tam sayı karekök fonksiyonu:

$$x_{n+1} = \frac{1}{2} \left( x_n + \frac{S}{x_n} \right)$$

Bu yaklaşım, her yinelemede doğru basamak sayısını ikiye katlayarak saniyeler içinde milyon basamaklı sayıların kökünü bulabilir.

## Çevre Dostu Önem Derecesi (Eco-Friendly Note)

Bu proje, Python veya Java gibi dillerin aksine arkada sürekli çalışan bir Çöp Toplayıcı (Garbage Collector) yükü barındırmaz. Doğrudan donanım seviyesinde makine koduna derlendiği için işlemci transistörlerini boşa çalıştırmaz, elektriği en verimli şekilde kullanır ve işini bitirir bitirmez işlemciyi serbest bırakır. 27 programlama dili arasında yapılan akademik enerji testlerinde en yeşil mimari seçilen C/C++ ve Rust felsefesine tam olarak sadık kalınmıştır.

Geliştirici: Ahmet Akif Abuş
