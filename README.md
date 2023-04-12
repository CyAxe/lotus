<p align="center">
<p align="center">
<img src="https://user-images.githubusercontent.com/45688522/222855427-5a328880-1ded-4adf-ac8e-35d0ba1c8cdd.png" width="470px" >
<!-- img src="https://user-images.githubusercontent.com/45688522/222831284-09899d3b-322b-4215-ba99-8294d4bf8a9b.png" width="170px" -->
</p>
Lotus is an automation system for Dynamic Application Security Testing (DAST) that simplifies web security scripting by providing a Lua API with numerous functions that can be used to automate web security processes in just a few lines of code

Our aim is to make security scripting more efficient by providing libraries and functions to ensure that no critical security features are missed in most security scripting cases.

### Contents
- [Why Lotus](#why-lotus)
- [What lotus scripts can do](#what-lotus-scripts-can-do)
- [Installation](#rocket-installation)
- [Documentation](#book-documentation)
- [Example](#example)
- [ScreenShots](#screenshots)

### Why Lotus?
- Powerful Lua API

    Lotus provides a comprehensive Lua API with numerous functions that can be used to automate web security processes. This makes scripting more efficient and allows for automation of complex security testing scenarios with ease.

- Enhanced Performance

    Lotus is designed with performance in mind, providing unparalleled speed and accuracy in automating web security testing. This allows you to focus on identifying and fixing security vulnerabilities rather than wasting time on manual testing.

- Flexibility

    Lotus offers a highly flexible API that allows for easy customization of reporting and HTTP requests, as well as input handling. Furthermore, it provides a variety of functions for matching and validating data, which can help you to identify vulnerabilities with greater accuracy and effectiveness.

- Active Development

    Lotus is an open-source project that is actively maintained, This means that you can rely on ongoing support and updates to ensure that Lotus remains an effective tool for web security testing.

- Easy Installation

    Installing Lotus is quick and easy, and can be done using the source code or binary file suitable for your operating system. Additionally, Lotus provides comprehensive documentation to help you get started with using the tool.

- Collaboration

    Lotus encourages collaboration and welcomes contributions from the community. If you encounter any issues or have suggestions for improving Lotus, you can reach out to the development team via Github repository issues page or join the Discord server.


### What Lotus scripts can do
- Regex Matching: Lotus provides a regex API with all options available for regex matching.
- Easy String Matching: Lotus has simple and efficient string matching functions.
- Multi-Threading: Lotus has two threading managers, one with normal threading and the other with callbacks and functions that can scan things like race conditions or scan parameters faster.
- URL Parsing: Lotus allows you to extract parameters and set payloads for URL parsing.
- HTML Parsing/Searching: Lotus has CSS selector search functions and can generate CSS selectors patterns.
- OS Utilities: Lotus provides various OS utilities like file reading, logging, path joining, and sleep functions for the current thread.
- HTTP Requests: Lotus can send HTTP requests with all available options like multipart, all methods, and custom headers.
- Encode/Decode: Lotus provides functions for encoding and decoding base64, url, and more.

And with our Lua libraries on LuaRocks, you can do even more. 


## :rocket: Installation
You can install Lotus from the source code by running the following commands:
```bash
$ apt install libssl-dev pkg-config gcc git -y
$ cargo install --git=https://github.com/BugBlocker/lotus/
```

Before running the command, ensure that you have installed the openssl-dev package. If you encounter any challenges while compiling, please open an issue on our Github repository for assistance. Alternatively, you can download the binary file from [Github Release page](https://github.com/BugBlocker/lotus/releases) suitable for your operating system and run it directly.
You will then need to download the lua scripts from our [Github Repository](https://github.com/BugBlocker/lotus-scripts) as quick start 
we provide you a lot of examples and scripts for diffrent cases
after downloading the report you will have to run the following command
```
$ echo http://testphp.vulnweb.com/listproducts.php?cat=1 | lotus scan lotus-scripts/active -o test_out.json -v
```


### :book: Documentation
You can find the documentation in the `docs/*.md` directory of the repository. However, if you prefer a more user-friendly and accessible version, you can visit our web version of the documentation at https://lotus.knas.me.
