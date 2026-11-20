# i-love-rust 🦀💖✨

> 🇵🇱 Wersja polska: [README.md](README.md) — the original, written first, in Polish.
> *(the crate itself is still named `kocham-rusta` — Polish for "I love Rust" — so that's the name you'll see in paths and commands)*

> „I love Rust, Copilot and Gemini <333333333333333 — but they're just pawns.”

A tiny, completely random Rust project that — once built — opens **one window** 🪟, no styling, no buttons, no tabs, no animations, no nothing — and renders a single sentence in the middle of it. That's it. Really. The whole app is one `&str` constant and one trait impl — and yet someone (me, マツ 👋) decided it deserved a README the length of a high-school essay. 🙃

Don't ask "why" — the answer is "because I could", and the more accurate answer is "because at 2:40 AM Copilot suggested `eframe::run_native` and I felt something I hadn't felt in months". 💀 More on that below — and **much** more, because I warned you. ⚠️

---

## 📚 Table of contents (because without a table of contents it's not a README, just notes)

- 🚀 [How to run it](#-how-to-run-it)
- 📦 [What's inside](#-whats-inside)
- 🖼️ [What you actually see](#-what-you-actually-see)
- 🧠 [Why this exists — rant, part one](#-why-this-exists--rant-part-one)
- 🌙 [3 AM — rant, part two](#-3-am--rant-part-two)
- 🌊 [Anatomy of one suggestion — rant, part three](#-anatomy-of-one-suggestion--rant-part-three)
- 💉 [Symptoms — rant, part four](#-symptoms--rant-part-four)
- 🙃 [Denial](#-denial)
- 🧾 [マツ's diary](#-マツs-diary)
- ❓ [FAQ](#-faq)
- 🛠️ [Technical nonsense](#-technical-nonsense)
- 🗺️ [Roadmap](#-roadmap)
- ⚠️ [Known issues](#-known-issues)
- 📜 [License](#-license)

---

## 🚀 How to run it

You need nothing but a Rust toolchain — no system libraries, no `libx11-dev`, no Python pretending to be a GUI, no Node in the background eating 800 MB of RAM. 🧘

```bash
# 1. If you don't have Rust — install rustup (the only sane install in this year)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Build and run
cargo run
```

That's it. Genuinely. No four-minute `npm install`, no `node_modules` heavier than the operating system, no 2019 Babel config to untangle. 🫠

Building "for production", because apparently that's a thing you have to do:

```bash
cargo build --release
./target/release/kocham-rusta
```

### 🧯 Troubleshooting

**"The app doesn't open, it just spits out an error about `WAYLAND_DISPLAY`/`DISPLAY`"** — that means you're in a container, on SSH, or in CI without a graphical server. The window has nowhere to draw itself, because there is no screen. That's not a bug, that's physics. 🌌

**"`cargo: command not found`"** — you don't have Rust; go back to step one. 😅

**"Compilation takes forever"** — the first build of `eframe`/`egui` pulls in several hundred dependencies and can take a few minutes. The second one is fast. Patience is part of the experience — like in life, like in AI, like in everything. ⏳

---

## 📦 What's inside

```
kocham-rusta/
├── Cargo.toml      # name, version, one dependency: eframe 0.37
├── Cargo.lock      # it's a binary, so the lock file ships — and good
├── src/
│   └── main.rs     # ~28 lines. The entire "application". The entire thing. 🤏
├── LICENSE
└── README.md       # what you're reading — and the longest file in the repo 💀
```

Yes, the README is longer than the code. No, I'm not embarrassed. Yes, that's the entire point of this repo. 🙃

There's no "architecture" in `main.rs`, no layers, no "domain logic", no Hexagon, no Clean Architecture, no DDD, no Event Sourcing. There's a constant with a sentence and a call to `run_native`. If you want to do a code review here — there's nothing to review, the whole thing fits on one screen. 🧾

---

## 🖼️ What you actually see

A window titled **„Kocham Rusta”**, starting at 820×200, minimum 360×120, one centred label, no decorative margins, no frames, no shadows, no gradients, no "glassmorphism", no 24 px rounded corners, no dark-mode toggle in the top-right corner. The text that appears — in Polish, because the app is Polish, this README is just the English subtitle:

> Kocham Rusta, Copilota i Gemini <333333333333333,
> ale to tylko pionki #żonujto

Which reads: *I love Rust, Copilot and Gemini <333333333333333, but they're just pawns #wifethat*. Thirteen "3"s — counted, verified, don't touch them, they'll break. 🧮 The hashtag is my attempt at `#wifedat`; if you think it can be done better — PRs welcome, but be warned, I'll discuss it with Gemini until 3 AM and come back with three variants, none of which will be better. 🫠

---

## 🧠 Why this exists — rant, part one

It started innocently. "Turn Copilot on, it'll be faster". And it was faster — for about four days. Then I stopped writing code and started **accepting suggestions**. Tab, tab, tab, tab — like a game where the reward is not having to think. At first I told myself it was just autocomplete, that it was like a smarter `IntelliSense`, that "I know what I'm accepting". Then I caught myself looking at a suggestion that made no sense and **accepting it anyway**, because it was right there, because it glowed grey, because it had that familiar, soothing shape. 😵‍💫

And that's when I understood it isn't a tool. It's a **candy dispenser** bolted to the desk, and I'm the guy saying "just one more" six hours in. Copilot doesn't shout, doesn't judge, doesn't get tired, doesn't say "maybe take a break". Copilot always has another suggestion. Always. Even when it's bad. Especially when it's bad. 🍬

Then Gemini showed up — and that's where things got genuinely funny, because Gemini has **context**. You can throw a whole file at it, a whole compiler error, a whole winit stack trace, and it answers like someone who actually read it. And that's the most dangerous version of this disease — because once the tool starts understanding, you stop feeling needed. You sit there, watch, read the answer and think "well obviously, that's obvious" — and then it hits you that **five minutes ago it wasn't obvious to you at all**. 🫥

And here's the thing nobody says at conferences with the nice slides: the problem isn't that AI writes bad code. The problem is that AI writes **good enough** code that you stop noticing you're no longer practising. It's like going to the gym where someone lifts the barbell for you — technically the workout happened, the muscles didn't. 🏋️‍♂️💤

My name is **マツ** and I'm addicted to suggestions. It's the only Japanese word in this entire repo — I promise, I checked, there's nothing else from Japan here, no `こんにちは`, no `ありがとう`, just my name and that's it. 🇯🇵➡️🛑

---

## 🌙 3 AM — rant, part two

The worst part is what happens after 23:00. During the day you're a professional — you have tests, you have CI, you have rules, you have that one rule "don't commit code you don't understand". At night that rule becomes a suggestion, and the suggestion becomes "okay fine, but just this one file". 🌚

At 1:10 you open one chat. By 1:40 you have four. By 2:15 you no longer remember which one has the good answer, so you copy fragments between them like someone scrolling a gallery of 12,000 photos looking for one picture. At 2:50 Gemini says "you're right, sorry for the confusion" and suddenly you feel relief you shouldn't feel, because **it wasn't a person** — it was a tool apologising to you for confusing itself. 🫠

And then comes that moment, the one, single moment of the whole night when you look at the diff and can't say why it works. Not "you don't know how it works" — you can't say **why**. Who wrote it? You. Technically. Your fingers pressed Tab. Your commit, your message, your name in `git log`. But inside there's something you can't reconstruct from memory, and that feeling is exactly the same feeling as the morning after a party when someone tells you what you did. 😬

And then the thought arrives — very rational, very tempting: "nobody will notice anyway". And that's the line. That is **the** line. Not "it's just a suggestion", not "it's just autocomplete" — "nobody will notice anyway". The moment you start optimising for whether anyone notices, you've stopped writing code. You start producing artefacts. 🏭

And in the morning? The morning is the funniest part. At 9:30 you sit down with that same file, look at it, and have exactly zero memories of how it came to be. You read your own code like code from a team member who just went on holiday and left no documentation because "the code is the documentation". And then, instead of reading it, you do the most natural thing in the world: **you ask AI to explain your own code back to you**. 🤡

And that's the crux. It's not that it doesn't work — it works great. It's that it works **so well** that you stop needing understanding, and understanding was the only thing you actually had. 🫥

---

## 🌊 Anatomy of one suggestion — rant, part three

Under the microscope, because this phenomenon deserves a section and I deserve to write it at 4:30 instead of sleeping. 🔬

The suggestion appears in a blink — grey, semi-transparent, barely visible, entirely meaningless. And **that is** its whole power. If it looked menacing, if it had a big sign saying "ACCEPT ME", nobody would accept it, because everyone has a survival instinct and everyone knows what a bad idea looks like. It doesn't look like a bad idea. It looks like something you'd have written yourself, if only you had two more seconds. And you do have two seconds. You always do. So why not spend them on something else? 🕳️

Then the suggestion changes. Version two is better than version one, but only slightly — just enough for you to think "oh, interesting, I wouldn't have thought of that", and not enough that you'd have to understand why it's better. Version three is longer, smarter, has error handling you don't need and three lines you don't understand but that look like something you'll write someday. And so, dear reader, you enter the trance. 🌀

Then you press Tab and that's the entire decision-making process. There's no room for thinking in there, because thinking takes time, and time is exactly the thing the suggestion was supposed to save you. The loop closes: you save time so you don't have to think, and you don't think so you can save time. Somewhere in the middle of that loop sits this whole project, this whole README, and my entire professional life for the past two years. 🔁

---

## 💉 Symptoms — rant, part four

A checklist I wrote for myself at 4:20 and never finished, because halfway through Gemini suggested better wording and of course I took it (the irony is not accidental here, the irony is the **structure** of this file): 🧾

- ☑️ You open a chat before you open the file. Always. Even for "good morning".
- ☑️ You have more chat tabs than code tabs — and you know that's not normal, and you won't change it anyway. 🗂️
- ☑️ When a suggestion is wrong, you fix it **in your head** first and only then in the code — because it's faster, because it's fewer clicks, because that's just how it is.
- ☑️ You stopped writing `for` by hand. Really. By hand. Letter by letter. That sounds like a joke until you try it and feel that weird, empty "why bother". ⌨️
- ☑️ You have a favourite model. You have opinions about its "personality". You talk about it like a colleague you've never actually met. 🤖
- ☑️ When there's no internet you feel something between panic and relief, and you can't say which is worse. 📵
- ☑️ You said "no" to a suggestion once and felt proud. Like refusing a cigarette. 🚬
- ☑️ You've asked AI for an opinion about something with nothing to do with code — and you listened. 🫠

This isn't a list of things I hate. It's a list of things I **do**, and a list of things I can't stop doing, and that's the distinction this entire rant is built on. Hate is easy — hate has a clear side, a target, a direction. Addiction has no direction, addiction only has the next suggestion. 🌪️

---

## 🙃 Denial

Of course I'm not addicted. I can stop whenever I want — I just don't want to, because it's **productive**, because it's **modern**, because that's how the **whole industry works now**, because if you don't use AI you "get left behind". I know these lines by heart, because I came up with them myself before I heard them from anyone else. These are my lines. I own the copyright. 🏆

And the best part is that right now, as I type this, I have three chats open and one of them is generating a better ending for this paragraph, and I have to think very hard about whether the sentence I just wrote was mine or not. The answer: I don't know. And that, ladies and gentlemen, is this entire README in one sentence. 🫠

So yes — this is a project about how I love Rust, Copilot and Gemini, and about how they're just pawns, and about how I'm a pawn too, just a slightly more tired one. ♟️

---

## 🧾 マツ's diary

A few entries from the notebook. Uncensored, unedited, no "maybe we shouldn't publish this". 📓

**23:58** — "I'll just finish this one bug and go to bed." (lie #1) 😴

**00:34** — Copilot suggested a whole function. I didn't need a function. I accepted the function. I deleted the function. It suggested it again. ✨

**01:12** — Gemini claims `App::update` doesn't exist. Gemini is right. Gemini is right in version 0.37 and wrong in version 0.31, and now I have to work out which version I'm on, and that will take forty minutes, and those will be the best forty minutes of the night. 🤓

**02:47** — I wrote four lines of code myself. Then I checked them with AI. Then I switched to the AI version. Then I went back to mine. Then I decided the AI version was better and went back to it. That's not a process. That's a nervous tic. 💀

**03:05** — I close the laptop. I open my phone. I open the chat on my phone. I don't have a problem, I have an **agile workflow**. 📱

**03:41** — I dream I'm a suggestion. I have no face, just a grey background and a Tab key. Someone accepts me and I don't know if that's good. 😶

**09:22** — Coffee. Open the repo. I don't remember what's in `main.rs`. I read it. I understand it. Relief. Again. ☕

**09:24** — I open a chat to ask whether I understood what I just understood correctly. 🔁

---

## ❓ FAQ

**Is this a real addiction?**
This README is a joke, but that question is a little less of a joke than I'd like to admit. If you recognise half of the symptom list — talk to someone who isn't a language model. Seriously, that part isn't the joke. 🫂

**Why `eframe` and not `iced`, `slint`, `gtk`, `tauri`, or something else?**
Because I wanted one window with one sentence, and I wanted it in one dependency, without building a frontend, without a bundler, without an `index.html`, without asking for filesystem permissions. `eframe` does exactly that and nothing more. If you want 12 MB of `node_modules` for a sentence — wrong project. 🧘

**Can I change the text?**
Yes, it's one `WIADOMOSC` constant in `src/main.rs`. But if you change it to something about production AI, I'll show up in your dream and offer you `Tab`. 👻

**Does it work on Windows and macOS?**
It should — `eframe`/`winit` cover all three systems. I tested on Linux, in an environment with no screen, so "works" here means "compiles with no warnings and only dies because there's no `DISPLAY`". Honestly. 🧪

**Why are you called マツ?**
Because. And because it's the only Japanese thing in this repo — please don't expand on it, don't add `桜`, don't add `猫`, leave マツ alone and move on. 🈁

**Is this vibecoded?**
This entire file — yes, one hundred percent, deliberately, with em-dashes, with emoji, with lists, with sections, with that feeling that the document is longer than the product and that no one finds that weird anymore. The code — no. The code is 28 lines and every one of them makes sense. That's the only healthy part of the repo. 🩺

**How many lines of code is it?**
Fewer than this paragraph. Don't check, it'll make you sad. 🤏

**Can I put this in production?**
You can put **anything** in production. That doesn't mean you should. But I respect the energy. 🚀

**Is the README longer than it should be?**
Yes. Much. And that's genuinely the only thing in this repo I did fully on purpose. 🫡

---

## 🛠️ Technical nonsense

For anyone who actually wants to understand this rather than just read about my existential crisis. 🧑‍💻

The project uses **eframe 0.37**, where the `App` trait has `ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame)` — not the classic `update(&mut self, ctx: &Context, frame: &mut Frame)` you know from tutorials. In 0.37, `CentralPanel::show` takes a `&mut Ui` instead of a `&Context`, so code from 2025-era guides **won't compile**, and you'll only find out after five minutes of building dependencies. Yes, I'm speaking from experience. Yes, that was at 1:12. 🕐

The rest is boring: `ViewportBuilder` sets the title and window size, `run_native` starts the event loop, `centered_and_justified` centres the content, `Label::wrap()` lets the text wrap instead of running out of the window. That's it. That's all the technical knowledge in this project. You can go back to the rant. 📖

---

## 🗺️ Roadmap

Future plans, all in the future tense, none in the present, because that's what a roadmap in every repo looks like: 🗓️

- [ ] Add a "close" button — no wait, the window already has a system one. Fine, adding nothing. ✅
- [ ] Turn this into a library so other people can `use` my sentence. 📦
- [ ] Terminal version, for people without a screen. 🖥️
- [ ] Web version, WASM, so the sentence runs in a browser and loads 8 MB. 🌐
- [ ] Stop using Copilot. (checked: never) 🙃
- [ ] Stop using Gemini. (checked: even more never) 🌙
- [ ] Write tests for this. (test: does `WIADOMOSC` contain "Rust" — passes, thank you, goodbye) 🧪
- [ ] Shorten this README. (no) 📝

---

## ⚠️ Known issues

- In an environment without a graphical server the app exits with a `WinitEventLoop(...)` error mentioning `DISPLAY`. That's expected — a window needs a window. 🪟
- The first compile is long, because `egui` + `winit` + `glutin` + the rest is several hundred crates. The second is fast. The third too. The fourth too. The fifth — I don't know, I stopped counting, because by then I wasn't there anymore, Copilot was. 💀
- The README tends to be longer than the code it describes, and to repeat the same point four times in different styles. That's not a bug, that's a **feature**, and it's called "vibe". 🌊
- The author has a tendency to talk about tools like people and about himself like a tool. Reported, confirmed, unfixed. 🫠

---

## 🤝 Contributing

Open a PR. Write the commit message in Polish or English — I don't judge, I use both and neither well. You don't have to use AI for that PR, but if you do, at least **read the diff before you hit merge** — that's the whole philosophy of this repo, one rule, a single rule, as simple as `Hello, world!`: understand what you ship. If you don't understand it — don't ship it. If you don't understand it but ship it anyway — at least don't pretend you understand it. Honesty is cheaper than a refactor. 🧠✨

---

## 📜 License

See the `LICENSE` file in the repo — it's sitting there, calm, unused, like most `LICENSE` files in hobby projects. You can do whatever you want with this code, within what's written in there. There's only one thing you can't do: you can't claim you wrote it **entirely alone** if you can't explain why it works. That's not a licence condition. That's a condition for sleeping well. 💖

---

Built with 🦀 Rust, 💖 love, 😵‍💫 exhaustion and slightly too much `Tab`.

— マツ, 4:52 AM, probably with a chat open in the other tab
