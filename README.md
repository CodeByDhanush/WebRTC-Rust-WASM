# Rust and WebAssembly (WASM)

Rust is a systems programming language known for its performance, safety, and concurrency features. WebAssembly (WASM) is a binary instruction format designed to run in web browsers, alongside JavaScript, enabling high-performance applications on the web.

---
## How Rust and WebAssembly Work Together

1. **Compilation to WebAssembly**: Rust code can be compiled to WebAssembly using tools like `wasm-pack` or directly with `rustc`. This process produces a `.wasm` file containing the binary instructions that can run in the browser.

2. **WebAssembly Modules**: These `.wasm` files are modular and self-contained units of code that can be loaded and executed by web browsers. They provide a compact and efficient representation of Rust functions, suitable for web delivery.

3. **JavaScript Interoperability**: Rust code can interact with JavaScript through a well-defined interface. This allows Rust functions to be called from JavaScript and vice versa, enabling seamless communication between the two languages.

4. **Performance Benefits**: By leveraging Rust's performance optimizations and compiling to WebAssembly, web applications can achieve near-native speeds for computationally intensive tasks. This makes Rust and WebAssembly a compelling choice for games, multimedia applications, simulations, and more.

---

# WebRTC Terminologies

WebRTC (Web Real-Time Communication) is a free, open-source project that provides web browsers and mobile applications with real-time communication capabilities via simple application programming interfaces (APIs). Here are some key terminologies:

---

## Signaling

Signaling is the process of coordinating communication between peers. This includes the exchange of session control messages such as offers, answers, and ICE candidates.

---

## ICE (Interactive Connectivity Establishment)

ICE is a framework used in WebRTC to establish a connection between two peers. It helps in finding the best path for media traffic and dealing with firewalls and NATs.

---

## SDP (Session Description Protocol)

SDP is a format for describing streaming media initialization parameters. In WebRTC, SDP is used to negotiate media capabilities and connection settings between peers.

---

## STUN (Session Traversal Utilities for NAT)

STUN is a protocol used to discover the public IP address of a peer and determine the presence of NATs or firewalls. It helps in establishing direct peer-to-peer connections.

---

## TURN (Traversal Using Relays around NAT)

TURN is a protocol used when direct peer-to-peer communication fails due to firewalls or NATs. It relays media streams through a third-party server to establish connectivity.

---

## NAT (Network Address Translation)

NAT is a technique used to modify network address information in IP packet headers while in transit. It allows multiple devices in a local network to share a single public IP address.

---

## MediaStream

MediaStream represents a stream of media data, such as audio or video. It is used in WebRTC to handle local audio and video streams from the user's device.

---

## PeerConnection

PeerConnection is the main object in the WebRTC API. It represents a connection between local and remote peers and handles sending and receiving audio, video, and data streams.

---

## DataChannel

DataChannel is a bidirectional communication channel between peers. It allows the exchange of arbitrary data, such as text or binary messages, in a peer-to-peer fashion.

---

## Constraints

Constraints are used to specify requirements for media streams and connections, such as video resolution, audio sample rate, or bandwidth limitations.

---

## Codec

Codec (Coder-Decoder) is a software component that converts audio and video data into a compressed format for transmission and then decodes it back into its original form.

---

This list provides an overview of some common WebRTC terminologies. For more in-depth information, refer to the official WebRTC documentation and specifications.


## Resources

- [Official Rust and WebAssembly Book](https://rustwasm.github.io/book/)
- [wasm-pack](https://github.com/rustwasm/wasm-pack): Tool for building, testing, and publishing Rust-generated WebAssembly
- [rustwasm GitHub Organization](https://github.com/rustwasm): Community-driven organization for Rust and WebAssembly projects

