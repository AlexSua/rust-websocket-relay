<a name="readme-top"></a>
<!-- PROJECT LOGO -->
<div align="center">

<h1 align="center">rust-websocket-relay</h2>

  <p align="center">
    A lightweight and resource-friendly relay server to easily communicate two applications using WebSockets.
    <br />
    <br />
 <div align="center">

[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]



</div>
    <a href="https://github.com/AlexSua/rust-websocket-relay/issues">Report Bug</a>
    ·
    <a href="https://github.com/AlexSua/rust-websocket-relay/issues">Request Feature</a>
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#build-the-project">Build the project</a></li>
      </ul>
    </li>
    <li>
	<a href="#usage">Usage</a>
	<ul>
        <li><a href="#run-server">Run server</a></li>
        <li><a href="#websocket-server-messages">Websocket server messages</a></li>
      </ul>
	</li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>
</br>

<!-- ABOUT THE PROJECT -->
## About The Project

Looking for a way to initialize a WebRTC connection, I came up with the solution of creating a simple WebSocket server where communication between two application clients A and B can be established by only using a string as an identifier and WebSocket as the communication protocol.

The functionality is simple:

1. Client A generates an ID and creates a WebSocket connection to the server with this ID.
2. The server gets the ID created by Client A and this ID together with a reference of the WebSocket connection is saved into a thread-safe map.
3. Client B creates another WebSocket connection using the same ID as Client A.
4. The server accepts the connection with Client B and checks if the ID that client B provides is already stored in the atomic map. If the ID exists in the map then the server gets the reference of A and communicates to Client A that B is connected.
5. Since the moment the communication between A and B is established the hashmap reference is deleted and every message sent through WebSocket will be relayed to the other peer directly whilst a mutual reference is kept between them.
6. If one of the clients disconnects from the server, the other client will update the atomic map with the ID and the WebSocket connection reference, waiting for another peer to connect, or, if a client C was already waiting with the same ID, it will be notified as shown in step 2 repeating the process shown above.

The result is a lightweight server, that allows a simple yet powerful way to communicate two applications. It only consumes resources keeping the WebSocket connection and WebSocket references during the connection time.

### Built With

Technologies used to develop this server were specifically chosen to allow the software to be highly performant.

* [![Rust][Rust-shield]][Rust-url]
* [![Actix][Actix-shield]][Actix-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- GETTING STARTED -->
## Getting Started

The necessary steps to build and run the application are shown below.

### Prerequisites

Before building the application you need to have installed the Rust compiler and the Rust package manager, Cargo. You can follow [this guide](https://doc.rust-lang.org/cargo/getting-started/installation.html) for more information about how to set up Rust.

### Build the Project

The steps to build the application:

1. Clone the repo

   ```bash
   git clone https://github.com/AlexSua/rust-websocket-relay.git
   ```

2. Enter the project directory:

   ```bash
   cd rust-websocket-relay
   ```

3. Build a release version of the project.

   ```bash
   cargo build --release
   ```

4. You can now execute the generated application:

   ```bash
   ./rust-websocket-relay
   ```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Usage
### Run server
You can have information about how to run the server with:

   ```bash
   ./rust-websocket-relay --help
   ```

By executing the command without any argument you get a WebSocket endpoint on

 ```bash
 ws://0.0.0.0:8080/<id>
 ``` 
where:
- **\<id\>**: Websocket identifier used to establish connection between two peers.

### Websocket server messages
During the WebSocket connection, you can get the following messages from the server:

- **/remote:open**: A remote peer has connected successfully using the same ID.
- **/remote:close**: The remote peer has closed the connection.
- **/status:connected**: Both peers are connected.
- **/status:waiting**: Waiting for a peer to be connected.
- **/error:code:error_message:message_sent**: There is an error sending a message.
  
<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ROADMAP -->
## Roadmap

* [x] Command line interface to configure the IP where the server is going to listen, port and other parameters.
* [x] SSL support.
* [ ] Decentralized architecture. Make the server a node that can be connected and synchronized with other nodes.
* [ ] Add multi-user rooms.

See the [open issues](https://github.com/AlexSua/rust-websocket-relay/issues) for a full list of proposed features (and known issues).

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTACT -->
## Contact

Alejandro Suárez - alejandrosuarez.eu@gmail.com

Project Link: [https://github.com/AlexSua/rust-websocket-relay](https://github.com/AlexSua/rust-websocket-relay)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Actix framework](https://actix.rs/)
* [Rust programming language](https://www.rust-lang.org)


<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[issues-shield]: https://img.shields.io/github/issues/AlexSua/rust-websocket-relay?style=flat-square
[issues-url]: https://github.com/AlexSua/rust-websocket-relay/issues

[license-shield]: https://img.shields.io/github/license/AlexSua/rust-websocket-relay?style=flat-square
[license-url]: https://github.com/AlexSua/rust-websocket-relay/blob/master/LICENSE.txt

[rust-shield]: https://img.shields.io/badge/rust-000000?style=for-the-badge&logo=rust&logoColor=white
[rust-url]: https://www.rust-lang.org

[actix-shield]: https://img.shields.io/badge/actix-ffffff?style=for-the-badge&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAGQAAABkBAMAAACCzIhnAAAABGdBTUEAALGPC/xhBQAAAAFzUkdCAK7OHOkAAAAbUExURUdwTGhnZmhnZmhnZmhnZmhnZmhnZmhnZmhnZhj9Cg4AAAAIdFJOUwDKoxrnWjh9efodmgAABGlJREFUWMPtWL9z2zYUhoSK6ig7dY6j5NRnjpTrNh5l5xxjpOso4Ug7TU4jkygtx5AmJfzZBUCQeA8EZeUuYzBRBD+83997ECE/149ZZ1NC337H935C4g0Zl/t+/xchfEqyigw4IbNoD8RInM0WJCjIBSeUp/tAeEp5+Y7zE8aTIY/Iy3A3YEk8vn3geh1lPCHBYidizBOPoxWNntDN46/vMWQbCEHeLgzTX5YHx7PmOSRBr6DbUEMO3yqTP3z25a902Kub0GokPynu2lf0k/zNeK/T1Jn8Bu2/UYcQ0gOK5fa19TKQPiDejSuEqYIU9nFD8TIPszLpQlb5UtieJz3qVg4p52on7VGXT5y5JdYzx8avdRb0RDEPeyEpXVubXkh7hIgaUImzFkWE3xdrkSVO52c6b2wV4j6vEMp4qSCvOlkv1tQFGfBteO90AOvzSiBP8tWu1wmYk1DmXNSLsuf6gQFr6L+edLEru5msF5VoWI1x7RIHwlcZoYv7tZV6Lh8rhMjUuWM/cyZYjeDayaIE4PqlI1islz5mjsJyfif3vE/g65M/JLedztrIeR+1R/IW4/3NAEJW6ko+PFoyFLcslx/ff/j8G9JIZZcsDlOAda4WYczd61VDApHlmQnx3Ajt3AsUOOWwpA2ytaomeLkdSgy5+jprHqcNM/KTZWvMSgeSIR+dtZxcLwCnBzpSZyZqYcuUJui+IZqR/u7SqFXvUExHGS+/gAS3VgJSLzGQClMY6kMgxgXI3YWVyIfHrbQNOGsDKKXCkCOh+1pDFm3HgcwqkuPyRZPlbYjfoEgo+yNEXBtQxpXpJuBkxCGx4dMx0KX2HlAM1NWKF433vBnQReG/mR8bCFmg5tbW/T/AyUPUV2IIWQEI9Q2XxIhCMqMYliL5UaspGJwnSJey+ewWE5LfGD3HpOsDk33M+rJ6nmtPgKoSQcqPI9DaJ1arP1yqaWQLhw+gJb03fjV8i7JNCYWFTK0OZmjyG6LHBDXwvDsDWO1eUBdw8j1HJwB2SnG6PwK9rEaZOSCizbIFapSPxOWACCofgRFreWt1D+p3IMJ63Evs5rrqQGJsrqK11KVZAh1/bX+BpgdqQ0Z2Z8w6fS3A5CpDlaMuF3cmjgsL4gvS/RONMHeB1V3HWLEhSrd2UEN9nTLksdhWXM92k64xU3BA6hiHi+4UugAN2BrXKLfzTIgtAm2ASIby/doeYrNTH4sZiN5N9Ztz5wxPlVufwegupHbS/hFzGN+0ZTCmD6WaFzJb6x4f9V0StgAyUdl5pdrO751p1BT8tRFamVLbkMx165m/UBE/atO/jM4Ma1D3/aX29HMY/S432Ter+jYZatJtJw2W998qL5VB+el/MygjnPdC6EF47hh7ckJ33UaHXQB74g4/4IfWEJoOeLIbkhOomxh3Cb164ip+JwTdtHNZuuLhHhf+gbzmV2LIfSd8O35CKx2dU8FAU8IqEgvfftnz7wuPJSR4JIOb7/tX5SElP9cPWv8DYQXyb6Cr4TQAAAAASUVORK5CYII=&logoColor=000000
[actix-url]: https://actix.rs/
