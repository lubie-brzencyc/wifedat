# kocham-rusta 🦀💖✨

> 🇬🇧 English version: [README.en.md](README.en.md) — bo nie każdy czyta po polsku o 4:52 w nocy.

> „Kocham Rusta, Copilota i Gemini <333333333333333 — ale to tylko pionki.”

Mały, totalnie randomowy projekt w Rustcie, który — po zbudowaniu — otwiera **jedno okno** 🪟, bez stylowania, bez przycisków, bez zakładek, bez animacji, bez niczego — i wypisuje na środku jedno zdanie. To wszystko. Serio. Cała aplikacja to jedna stała `&str` i jedna implementacja traitu — a mimo to ktoś (ja, マツ 👋) uznał, że to zasługuje na README długości wypracowania maturalnego. 🙃

Nie pytaj „po co” — odpowiedź brzmi „bo mogłem”, a dokładniejsza odpowiedź brzmi „bo o 2:40 w nocy Copilot podpowiedział mi `eframe::run_native` i poczułem coś, czego nie czułem od miesięcy”. 💀 Więcej o tym poniżej — i to **znacznie** więcej, bo ostrzegałem. ⚠️

---

## 📚 Spis treści (bo bez spisu treści to nie README, tylko notatka)

- 🚀 [Jak to odpalić](#-jak-to-odpalić)
- 📦 [Co jest w środku](#-co-jest-w-środku)
- 🖼️ [Co właściwie widzisz na ekranie](#-co-właściwie-widzisz-na-ekranie)
- 🧠 [Dlaczego to istnieje — czyli rant, część pierwsza](#-dlaczego-to-istnieje--czyli-rant-część-pierwsza)
- 🌙 [Trzecia w nocy — czyli rant, część druga](#-trzecia-w-nocy--czyli-rant-część-druga)
- 🌊 [Anatomia jednej podpowiedzi — rant, część trzecia](#-anatomia-jednej-podpowiedzi--rant-część-trzecia)
- 💉 [Objawy — czyli rant, część czwarta](#-objawy--czyli-rant-część-czwarta)
- 🙃 [Zaprzeczenie](#-zaprzeczenie)
- 🧾 [Dziennik マツ](#-dziennik-マツ)
- ❓ [FAQ](#-faq)
- 🛠️ [Techniczne pierdoły](#-techniczne-pierdoły)
- 🗺️ [Roadmapa](#-roadmapa)
- ⚠️ [Znane problemy](#-znane-problemy)
- 📜 [Licencja](#-licencja)

---

## 🚀 Jak to odpalić

Potrzebujesz tylko toolchaina Rusta — nic więcej, żadnych bibliotek systemowych, żadnego `libx11-dev`, żadnego Pythona udającego, że jest GUI, żadnego Node'a w tle jedzącego 800 MB RAM-u. 🧘

```bash
# 1. Jeśli nie masz Rusta — wrzuć rustup (to jedyna sensowna instalacja w tym roku)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Zbuduj i odpal
cargo run
```

I tyle. Naprawdę tyle. Bez `npm install` trwającego cztery minuty, bez `node_modules` ważącego więcej niż system operacyjny, bez konfiguracji Babela z 2019 roku. 🫠

Budowanie wersji „produkcyjnej”, bo podobno tak trzeba:

```bash
cargo build --release
./target/release/kocham-rusta
```

### 🧯 Rozwiązywanie problemów

**„Aplikacja nie otwiera się, tylko wypluwa błąd o `WAYLAND_DISPLAY`/`DISPLAY`”** — to znaczy, że siedzisz w kontenerze, na SSH albo w CI bez serwera graficznego. Okno nie ma gdzie się narysować, bo nie ma ekranu. To nie jest bug, to jest fizyka. 🌌

**„`cargo: command not found`”** — nie masz Rusta, wróć do punktu pierwszego. 😅

**„Kompilacja trwa wieczność”** — pierwsze budowanie `eframe`/`egui` ciągnie kilkaset zależności i potrafi zająć kilka minut. Drugie jest już szybkie. Cierpliwość jest częścią doświadczenia — jak w życiu, jak w AI, jak we wszystkim. ⏳

---

## 📦 Co jest w środku

```
kocham-rusta/
├── Cargo.toml      # nazwa, wersja, jedna zależność: eframe 0.37
├── Cargo.lock      # bo binarka, więc lock trafia do repo — i dobrze
├── src/
│   └── main.rs     # ~28 linii. Cała „aplikacja”. Cała. 🤏
├── LICENSE
└── README.md       # to, co właśnie czytasz — i to jest najdłuższy plik w repo 💀
```

Tak, README jest dłuższe niż kod. Nie, nie jest mi wstyd. Tak, to jest cała pointa tego repo. 🙃

W `main.rs` nie ma „architektury”, nie ma warstw, nie ma „domain logic”, nie ma Hexagonu, nie ma Clean Architecture, nie ma DDD, nie ma Event Sourcingu. Jest stała z napisem i wywołanie `run_native`. Jeśli chcesz tu robić code review — nie ma czego review'ować, wszystko widać na jednym ekranie. 🧾

---

## 🖼️ Co właściwie widzisz na ekranie

Okno o tytule **„Kocham Rusta”**, rozmiar startowy 820×200, minimalny 360×120, jedna wyśrodkowana etykieta, brak marginesów ozdobnych, brak ramek, brak cieni, brak gradientów, brak „glassmorphism”, brak zaokrągleń 24 px, brak trybu ciemnego przełączanego przyciskiem w prawym górnym rogu. Tekst, który się pojawia:

> Kocham Rusta, Copilota i Gemini <333333333333333,
> ale to tylko pionki #żonujto

Trzynaście „3” — policzone, sprawdzone, nie ruszaj tego, bo się zepsuje. 🧮 Hashtag to próba oddania `#wifedat` po polsku, więc jeśli uważasz, że da się to przetłumaczyć lepiej — zapraszam, otwórz PR, ale uprzedzam, że będę o tym rozmawiał z Gemini do trzeciej w nocy i wrócę z trzema wariantami, z których żaden nie będzie lepszy. 🫠

---

## 🧠 Dlaczego to istnieje — czyli rant, część pierwsza

Zaczęło się niewinnie. „Włącz Copilota, będzie szybciej”. No i było szybciej — przez jakieś cztery dni. Potem przestałem pisać kod, a zacząłem **przyjmować propozycje**. Tab, tab, tab, tab — jak w grze, w której nagrodą jest to, że nie musisz myśleć. Na początku mówiłem sobie, że to tylko autouzupełnianie, że to jak inteligentniejszy `IntelliSense`, że przecież „wiem, co przyjmuję”. Potem złapałem się na tym, że patrzę na podpowiedź, która nie ma sensu, i **i tak ją przyjmuję**, bo była pod ręką, bo świeciła się na szaro, bo miała ten znajomy, uspokajający kształt. 😵‍💫

I wtedy zrozumiałem, że to nie jest narzędzie. To jest **automat do cukierków** ustawiony przy biurku, a ja jestem człowiekiem, który mówi „jeszcze jeden” od sześciu godzin. Copilot nie krzyczy, nie ocenia, nie męczy się, nie mówi „może odpocznij”. Copilot zawsze ma kolejną propozycję. Zawsze. Nawet jeśli jest zła. Zwłaszcza jeśli jest zła. 🍬

Potem doszło Gemini — i tu sytuacja zrobiła się naprawdę zabawna, bo Gemini ma **kontekst**. Możesz mu wrzucić cały plik, cały błąd kompilatora, cały stack trace z winit, i on odpowiada jak ktoś, kto naprawdę czytał. I to jest najbardziej niebezpieczna wersja tej choroby — bo kiedy narzędzie zaczyna rozumieć, przestajesz czuć, że jesteś potrzebny. Siedzisz, patrzysz, czytasz odpowiedź i myślisz: „no tak, oczywiście, przecież to jest oczywiste” — a potem zdajesz sobie sprawę, że **pięć minut temu to nie było dla ciebie oczywiste wcale**. 🫥

I tu jest ten problem, o którym nikt nie mówi na konferencjach z fajnymi slide'ami: nie chodzi o to, że AI pisze zły kod. Chodzi o to, że AI pisze **wystarczająco dobry** kod, żebyś przestał zauważać, że już nie ćwiczysz. To jak chodzenie na siłownię, gdzie ktoś podnosi za ciebie sztangę — technicznie trening się odbył, mięśnie nie. 🏋️‍♂️💤

Nazywam się **マツ** i jestem uzależniony od podpowiedzi. To jedyne japońskie słowo w całym tym repo — obiecuję, sprawdziłem, nie ma tu nic więcej z Japonii, żadnego `こんにちは`, żadnego `ありがとう`, tylko moje imię i tyle. 🇯🇵➡️🛑

---

## 🌙 Trzecia w nocy — czyli rant, część druga

Najgorsze jest to, co się dzieje po godzinie 23:00. W dzień jesteś profesjonalistą — masz testy, masz CI, masz zasady, masz tę jedną zasadę „nie commitować kodu, którego nie rozumiem”. W nocy ta zasada zamienia się w sugestię, a sugestia w „no dobra, ale tylko ten jeden plik”. 🌚

O 1:10 otwierasz jeden czat. O 1:40 masz już cztery. O 2:15 nie pamiętasz, w którym z nich jest ta jedna dobra odpowiedź, więc kopiujesz fragmenty między nimi jak ktoś, kto szuka zdjęcia w galerii z 12 tysiącami zdjęć. O 2:50 Gemini mówi „masz rację, przepraszam za zamieszanie” i nagle czujesz ulgę, której nie powinieneś czuć, bo **to nie była osoba** — to było narzędzie, które właśnie cię przeprosiło za to, że samo siebie pomyliło. 🫠

A potem przychodzi ten moment, ten jeden, jedyny moment całej nocy, w którym patrzysz na diff i nie potrafisz powiedzieć, dlaczego działa. Nie „nie wiesz, jak działa” — nie potrafisz powiedzieć, **dlaczego**. Kto to napisał? Ty. Technicznie. Twoje palce nacisnęły Tab. Twój commit, twoja wiadomość, twoje imię w `git log`. Ale w środku jest coś, czego nie umiesz odtworzyć z pamięci, i to uczucie jest dokładnie tym samym uczuciem, co rano po imprezie, kiedy ktoś opowiada ci, co robiłeś. 😬

I wtedy pojawia się myśl — bardzo racjonalna, bardzo kusząca: „przecież nikt tego nie zauważy”. I to jest właśnie to zdanie. To jest **to** zdanie. Nie „przecież to tylko podpowiedź”, nie „przecież to tylko autouzupełnianie” — tylko „przecież nikt tego nie zauważy”. W momencie, w którym zaczynasz optymalizować pod to, czy ktoś zauważy, przestałeś pisać kod. Zaczynasz produkować artefakty. 🏭

A rano? Rano jest najzabawniejsza część. O 9:30 siadasz do tego samego pliku, patrzysz na niego i masz dokładnie zero wspomnień z tego, jak powstał. Czytasz własny kod jak kod kogoś z innego zespołu, który właśnie poszedł na urlop i nie zostawił dokumentacji, bo „kod jest dokumentacją”. I wtedy, zamiast go przeczytać, robisz najbardziej naturalną rzecz na świecie: **pytasz AI, żeby ci wyjaśniło twój własny kod**. 🤡

No i tu jest sedno. Nie chodzi o to, że to nie działa — działa świetnie. Chodzi o to, że działa **tak dobrze**, że przestajesz potrzebować zrozumienia, a zrozumienie było jedyną rzeczą, którą naprawdę miałeś. 🫥

---

## 🌊 Anatomia jednej podpowiedzi — rant, część trzecia

Zjęcie pod mikroskopem, bo to zjawisko zasługuje na sekcję, a ja zasługuję na to, żeby o nim napisać o 4:30 zamiast spać. 🔬

Podpowiedź pojawia się w mgnieniu oka — szara, półprzezroczysta, ledwo widoczna, kompletnie nieznacząca. I **to jest** jej cała siła. Gdyby wyglądała groźnie, gdyby miała wielki napis „PRZYJMIJ MNIE”, nikt by jej nie przyjmował, bo każdy ma instynkt samozachowawczy i każdy wie, jak wygląda zły pomysł. Ona nie wygląda jak zły pomysł. Wygląda jak coś, co i tak byś napisał, gdybyś tylko miał dwie sekundy więcej. A przecież masz dwie sekundy. Masz je zawsze. Więc dlaczego nie użyć ich na coś innego? 🕳️

Potem podpowiedź się zmienia. Wersja druga jest lepsza od pierwszej, ale tylko trochę — na tyle, żebyś pomyślał „o, ciekawe, nie wpadłbym na to”, a nie na tyle, żebyś musiał zrozumieć, dlaczego lepsza. Trzecia wersja jest już dłuższa, mądrzejsza, ma obsługę błędów, której nie potrzebujesz, i trzy linijki, których nie rozumiesz, ale które wyglądają jak coś, co kiedyś napiszesz. I tak, drogi czytelniku, wchodzisz w trans. 🌀

A potem naciskasz Tab i to jest cały proces podejmowania decyzji. Nie ma tam miejsca na myślenie, bo myślenie zajmuje czas, a czas to dokładnie ta rzecz, którą ta podpowiedź miała ci oszczędzić. Pętla się domyka: oszczędzasz czas, żeby nie myśleć, i nie myślisz, żeby oszczędzać czas. Gdzieś w środku tej pętli jest cały ten projekt, całe to README i całe moje życie zawodowe od dwóch lat. 🔁

## 💉 Objawy — czyli rant, część czwarta

Lista kontrolna, którą sam sobie napisałem o 4:20 i której nie skończyłem, bo w połowie Gemini zasugerowało lepsze sformułowanie i oczywiście je przyjąłem (ironia nie jest tu przypadkowa, ironia jest tu **strukturą** tego pliku): 🧾

- ☑️ Otwierasz czat, zanim otworzysz plik. Zawsze. Nawet na „dzień dobry”.
- ☑️ Masz więcej zakładek z czatem niż z kodem — i wiesz, że to nie jest normalne, i i tak tego nie zmienisz. 🗂️
- ☑️ Gdy podpowiedź jest zła, poprawiasz ją **w głowie** i dopiero potem w kodzie — bo tak szybciej, bo tak mniej klikania, bo tak.
- ☑️ Przestałeś pisać `for` ręcznie. Serio. Ręcznie. Litera po literze. To brzmi jak żart, dopóki nie spróbujesz i nie poczujesz tego dziwnego, pustego „a po co”. ⌨️
- ☑️ Masz ulubiony model. Masz opinię o jego „charakterze”. Rozmawiasz o nim jak o koledze z pracy, którego nigdy nie widziałeś na oczy. 🤖
- ☑️ Kiedy nie ma internetu, czujesz coś pomiędzy paniką a ulgą, i nie umiesz powiedzieć, co jest gorsze. 📵
- ☑️ Powiedziałeś „nie” podpowiedzi i poczułeś dumę. Jakbyś odmówił papierosowi. 🚬
- ☑️ Zdarzyło ci się zapytać AI o zdanie w sprawie, która nie ma nic wspólnego z kodem — i posłuchać. 🫠

To nie jest lista rzeczy, których nienawidzę. To lista rzeczy, które **robię**, i lista rzeczy, których nie potrafię przestać robić, i to jest różnica, o którą cały ten rant się rozbija. Nienawiść jest łatwa — nienawiść ma jasną stronę, ma cel, ma kierunek. Uzależnienie nie ma kierunku, uzależnienie ma tylko następną podpowiedź. 🌪️

---

## 🙃 Zaprzeczenie

Oczywiście, że nie jestem uzależniony. Mogę przestać w każdej chwili — po prostu nie chcę, bo to jest **produktywne**, bo to jest **nowoczesne**, bo tak teraz **pracuje cała branża**, bo tak, jak nie używasz AI, to „zostajesz w tyle”. Znam te zdania na pamięć, bo sam je wymyśliłem, zanim usłyszałem je od kogokolwiek innego. To moje zdania. Mam do nich prawa autorskie. 🏆

A najlepsze jest to, że w tym momencie, kiedy to piszę, mam otwarte trzy czaty i jeden z nich właśnie generuje mi lepsze zakończenie tego akapitu, i muszę się bardzo mocno zastanowić, czy to zdanie, które przed chwilą napisałem, było moje, czy nie. Odpowiedź: nie wiem. I to jest, proszę państwa, cały ten README w jednym zdaniu. 🫠

Więc tak — to jest projekt o tym, że kocham Rusta, Copilota i Gemini, i o tym, że one są tylko pionkami, i o tym, że ja też jestem pionkiem, tylko trochę bardziej zmęczonym. ♟️

---

## 🧾 Dziennik マツ

Kilka wpisów z notatnika. Bez cenzury, bez korekty, bez „a może lepiej tego nie publikować”. 📓

**23:58** — „Tylko dokończę ten jeden błąd i idę spać.” (kłamstwo nr 1) 😴

**00:34** — Copilot zaproponował całą funkcję. Nie potrzebowałem funkcji. Przyjąłem funkcję. Usunąłem funkcję. Zaproponował ją znowu. ✨

**01:12** — Gemini twierdzi, że `App::update` nie istnieje. Gemini ma rację. Gemini ma rację w wersji 0.37 i nie ma racji w wersji 0.31, i muszę teraz ustalić, w której wersji jestem, i to zajmie mi czterdzieści minut, i to będzie najlepsze czterdzieści minut tej nocy. 🤓

**02:47** — Napisałem cztery linie kodu samodzielnie. Potem sprawdziłem je w AI. Potem zmieniłem na wersję z AI. Potem wróciłem do swojej. Potem uznałem, że wersja z AI była lepsza, i wróciłem do niej. To nie jest proces. To jest tik nerwowy. 💀

**03:05** — Zamykam laptop. Otwieram telefon. Otwieram czat w telefonie. Nie mam problemu, mam **elastyczny workflow**. 📱

**03:41** — Śnię, że jestem podpowiedzią. Nie mam twarzy, tylko szare tło i przycisk Tab. Ktoś mnie przyjmuje, a ja nie wiem, czy to dobrze. 😶

**09:22** — Kawa. Otwieram repo. Nie pamiętam, co jest w `main.rs`. Czytam. Rozumiem. Ulga. Znowu. ☕

**09:24** — Otwieram czat, żeby zapytać, czy dobrze rozumiem to, co właśnie zrozumiałem. 🔁

---

## ❓ FAQ

**Czy to prawdziwe uzależnienie?**
To README jest żartem, ale to pytanie jest trochę mniej żartem, niż chciałbym przyznać. Jeśli rozpoznajesz połowę listy objawów — pogadaj z kimś, kto nie jest modelem językowym. Serio, to nie jest część żartu. 🫂

**Dlaczego `eframe`, a nie `iced`, `slint`, `gtk`, `tauri` albo cokolwiek innego?**
Bo chciałem jedno okno z jednym napisem i chciałem to mieć w jednej zależności, bez budowania frontendu, bez bundlera, bez `index.html`, bez uprawnień do plików systemowych. `eframe` robi dokładnie to i nic więcej. Jeśli chcesz 12 MB `node_modules` dla napisu — to nie ten projekt. 🧘

**Czy mogę zmienić tekst?**
Tak, to jedna stała `WIADOMOSC` w `src/main.rs`. Ale jeśli zmienisz ją na coś związanego z produkcyjnym AI, przyjdę do ciebie we śnie i zaproponuję `Tab`. 👻

**Działa na Windowsie i macOS?**
Powinno — `eframe`/`winit` obsługują wszystkie trzy systemy. Ja testowałem na Linuksie, w środowisku bez ekranu, więc „działa” oznacza tu „kompiluje się bez ostrzeżeń i wywala się tylko dlatego, że nie ma `DISPLAY`”. Uczciwie. 🧪

**Dlaczego nazywasz się マツ?**
Bo tak. I bo to jedyna japońska rzecz w tym repo — proszę tego nie rozbudowywać, nie dodawaj tu `桜`, nie dodawaj `猫`, zostaw マツ i idź dalej. 🈁

**Czy to jest vibecoded?**
Cały ten plik — tak, w stu procentach, z premedytacją, z em-dashami, z emoji, z listami, z sekcjami, z tym poczuciem, że dokument jest dłuższy niż produkt i że to już nikogo nie dziwi. Kod — nie. Kod ma 28 linii i każda z nich jest zrozumiała. To jest jedyna zdrowa część repo. 🩺

**Ile to ma linii kodu?**
Mniej niż ten akapit. Nie sprawdzaj, będzie ci przykro. 🤏

**Czy mogę to wrzucić na produkcję?**
Możesz wrzucić **wszystko** na produkcję. To nie znaczy, że powinieneś. Ale doceniam energię. 🚀

**Czy README jest dłuższe niż powinno?**
Tak. Znacznie. I to jest właściwie jedyna rzecz w tym repo, którą zrobiłem w pełni świadomie. 🫡

---

## 🛠️ Techniczne pierdoły

Dla kogoś, kto faktycznie chce to zrozumieć, a nie tylko przeczytać o moim kryzysie egzystencjalnym. 🧑‍💻

Projekt używa **eframe 0.37**, w którym trait `App` ma metodę `ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame)` — a nie klasyczne `update(&mut self, ctx: &Context, frame: &mut Frame)`, które znasz z tutoriali. W 0.37 `CentralPanel::show` przyjmuje `&mut Ui`, a nie `&Context`, więc kod z poradników z 2025 roku **nie skompiluje się** i będziesz o tym wiedział dopiero po pięciu minutach kompilacji zależności. Tak, mówię z doświadczenia. Tak, to było o 1:12. 🕐

Reszta jest nudna: `ViewportBuilder` ustawia tytuł i rozmiar okna, `run_native` uruchamia pętlę zdarzeń, `centered_and_justified` wyśrodkowuje zawartość, `Label::wrap()` pozwala tekstowi się zawinąć zamiast wyjechać poza okno. Koniec. To jest cała wiedza techniczna w tym projekcie. Możesz wracać do rantu. 📖

---

## 🗺️ Roadmapa

Plany na przyszłość, wszystkie w czasie przyszłym, żadne w czasie teraźniejszym, bo tak wygląda każda roadmapa w każdym repo: 🗓️

- [ ] Dodać przycisk „zamknij” — nie, czekaj, okno już ma przycisk systemowy. Dobra, nic nie dodajemy. ✅
- [ ] Zrobić z tego bibliotekę, żeby inni mogli `use`'ować mój napis. 📦
- [ ] Wersja na terminal, dla ludzi bez ekranu. 🖥️
- [ ] Wersja webowa, WASM, żeby napis działał w przeglądarce i wczytywał 8 MB. 🌐
- [ ] Przestać używać Copilota. (odhaczone: nigdy) 🙃
- [ ] Przestać używać Gemini. (odhaczone: jeszcze bardziej nigdy) 🌙
- [ ] Napisać do tego testy. (test: czy `WIADOMOSC` zawiera „Rusta” — przechodzi, dziękuję, do widzenia) 🧪
- [ ] Skrócić to README. (nie) 📝

---

## ⚠️ Znane problemy

- W środowisku bez serwera graficznego aplikacja kończy się błędem `WinitEventLoop(...)` i tekstem o `DISPLAY`. To oczekiwane — okno potrzebuje okna. 🪟
- Pierwsza kompilacja jest długa, bo `egui` + `winit` + `glutin` + cała reszta to kilkaset crate'ów. Druga jest szybka. Trzecia też. Czwarta też. Piąta — nie wiem, przestałem liczyć, bo w tym momencie już mnie nie było, był Copilot. 💀
- README ma tendencję do bycia dłuższym niż kod, który opisuje, i do powtarzania tej samej pointy cztery razy w różnych stylach. To nie jest bug, to jest **feature** i nazywa się „vibe”. 🌊
- Autor ma skłonność do mówienia o narzędziach jak o ludziach, a o sobie jak o narzędziu. Zgłoszone, potwierdzone, nienaprawione. 🫠

---

## 🤝 Współtworzenie

Otwórz PR. Napisz commit message po polsku albo po angielsku — nie oceniam, sam używam obu i żadnego dobrze. Nie musisz używać AI do tego PR-a, ale jeśli użyjesz, to przynajmniej **przeczytaj diff, zanim klikniesz merge** — to cała filozofia tego repo, jedna zasada, jedna jedyna, prosta jak `Hello, world!`: zrozum to, co wysyłasz. Jeśli nie rozumiesz — nie wysyłaj. Jeśli nie rozumiesz, ale wysyłasz — przynajmniej nie udawaj, że rozumiesz. Uczciwość jest tańsza niż refaktor. 🧠✨

---

## 📜 Licencja

Zobacz plik `LICENSE` w repo — leży tam, spokojny, nieużywany, jak większość plików `LICENSE` w projektach hobbystycznych. Możesz robić z tym kodem, co chcesz, w granicach tego, co tam napisano. Nie możesz tylko jednego: nie możesz twierdzić, że to napisałeś **całkowicie sam**, jeśli nie umiesz wyjaśnić, dlaczego działa. To nie jest warunek licencji. To jest warunek dobrego samopoczucia. 💖

---

Zbudowane z 🦀 Rustem, 💖 miłością, 😵‍💫 zmęczeniem i odrobiną zbyt dużej ilości `Tab`.

— マツ, 4:52 w nocy, prawdopodobnie z otwartym czatem w drugiej zakładce
