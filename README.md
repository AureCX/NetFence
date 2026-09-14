# NetFence

NetFence is a small project I'm building to experiment with **website and domain filtering** on Windows and Android.

The idea is pretty simple: let the user define rules for websites they want to block, without having to rely entirely on predefined blocklists.

For example, you could have a rule like:

```text
*example*
```

which would block domains such as:

```text
example.com
www.example.com
sub.example.com
something-example.net
```

while still allowing unrelated domains.

The project is currently a **proof of concept**, and I'm using it to learn more about networking, DNS, Rust, Android networking, and how website filtering actually works at the system level.

---

## What I'm trying to build

The end goal is something along these lines:

```text
                    NetFence
                       │
              ┌────────┴────────┐
              │                 │
           Windows           Android
              │                 │
          DNS / WFP        VpnService
              │                 │
              └────────┬────────┘
                       │
                  Filter rules
```

The filtering rules themselves should be independent from the platform.

So ideally, a rule created on Windows could also be used by the Android version.

---

## Rules

The main idea is to make the rules flexible enough to cover simple and more specific cases.

For example:

```text
example.com
```

could match a specific domain, while:

```text
*example*
```

could match anything containing `example` in the hostname.

Eventually I'd like to support things such as:

```text
*.example.com
youtube.com/shorts/*
```

as well as allow rules:

```text
BLOCK  *example*
ALLOW  safe.example.com
```

This also means that more specific rules could override broader ones.

For example:

```text
BLOCK  *example*
ALLOW  safe.example.com
```

would give:

```text
example.com          → BLOCK
www.example.com      → BLOCK
danger.example.com   → BLOCK
safe.example.com     → ALLOW
```

---

## Current state

Right now, I'm starting with the part that doesn't require any system-level networking:

**the rule engine.**

The first version will be able to take a hostname and determine whether it should be allowed or blocked.

Something along the lines of:

```text
$ netfence check example.com

✗ BLOCKED
Rule: *example*
```

and:

```text
$ netfence check google.com

✓ ALLOWED
```

I'm implementing this first in **Python and Rust**.

The Python version is useful for quickly trying things out, while the Rust version is helping me learn Rust by implementing essentially the same thing from scratch.

---

## Why Python and Rust?

Mostly because I want to compare the two while building something real.

The Python implementation lets me experiment with the filtering logic without spending too much time on the language itself.

Then I can take the same ideas and implement them in Rust:

```python
result = filter.check("example.com")
```

versus:

```rust
let result = filter.check("example.com");
```

The long-term plan is for the Rust implementation to become the main filtering engine.

---

## Where this is going

Once the basic rule engine works, I want to start actually filtering network traffic.

The first thing I'll probably experiment with is **DNS filtering**:

```text
Browser
   │
   │ DNS request
   ▼
NetFence
   │
   ├── blocked → reject
   │
   └── allowed → forward
```

This should work well for the type of domain-based filtering I'm interested in.

After that, I'd like to look into the platform-specific parts:

### Windows

* Local DNS filtering
* Windows service
* Windows Filtering Platform (WFP)
* Desktop UI

### Android

* `VpnService`
* Local DNS filtering
* Android UI

### Browser extension

A browser extension could eventually handle things that are specific to the browser, such as URL-based rules.

---

## Planned features

Nothing here is set in stone yet, but these are some of the things I'd like to experiment with:

* [ ] Domain matching
* [ ] Wildcard matching
* [ ] Allow / block rules
* [ ] Rule priorities
* [ ] Python implementation
* [ ] Rust implementation
* [ ] CLI
* [ ] Local DNS filtering
* [ ] Windows support
* [ ] Android support
* [ ] Browser extension
* [ ] URL-based rules
* [ ] Scheduled rules
* [ ] Per-application rules
* [ ] Filtering statistics

---

## Project structure

The project is still taking shape, but the idea is roughly:

```text
NetFence/
├── python/
│   ├── main.py
│   ├── filter.py
│   └── rules.json
│
├── rust/
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       └── filter.rs
│
└── README.md
```

This will probably change quite a bit as the project grows.

---

## Privacy

One of the things I'd like to keep important throughout the project is **privacy**.

The goal is for NetFence to work locally as much as possible, without sending browsing activity to an external server just to decide whether a website should be blocked.

---

## Status

🚧 **Early development / POC**

This is currently more of a learning project than a finished application.

I'm starting with the rule-matching system and will build the networking parts once that foundation is working properly.

---
