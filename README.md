<p align="center">
<img src="https://user-images.githubusercontent.com/45688522/187603703-5781b86b-9f5a-4658-9370-7083a3b5b6d5.png" width="470px">
</p>


:warning: We're Looking for maintainers
https://github.com/rusty-sec/lotus/issues/39

# lotus


:zap: Fast Web Security Scanner written in Rust based on Lua Scripts :waning_gibbous_moon: :crab: 


Here at Lotus, we strive to make this process of automating your own web security module as simple and fast as possible. 
There is still a lot of development taking place on this project at the moment, as there are a lot of features that haven't been implemented yet such as (OAST, reading headers, reading raw requests instead of URLs, crawler, custom report script, etc.), so we would appreciate any contributions you may be able to provide to this project so that it can be finalized sooner. If you have any additional questions, please feel free to visit the github repo issues page and join our [Discord Server](https://discord.gg/nBYDPTzjSq)

### Usage
It can be built from source, but ensure that you install the package `openssl-dev` before running this command

```bash
$ cargo install --git=https://github.com/rusty-sec/lotus/
```
You will then need to download the lua scripts from our [github repository](https://github.com/rusty-sec/lotus-scripts) and run the following command

```
$ echo http://testphp.vulnweb.com/listproducts.php?cat=1 | lotus --scripts lotus-scripts/active --output test_out.json
```

### Lua API
We are working on creating a new document for this list as soon as possible, as it has not been updated for a long time 

| Function   |      About      |  output type | Example |
|----------|:-------------:|------:| -----:|
| is_match |  check if regex is matching with the text or not | bool | `is_match("\d\d\d","123") -- true` |
| println |    print message above the progress bar   | Nil | `println("XSS FOUND :D")` |
| log_info | logging with info level | Nil | `log_info("Hello")`|
| log_debug | logging with debug level | Nil | `log_debug("Hello")`|
| log_warn | logging with warn level | Nil | `log_warn("Hello")`|
| log_error | logging with error level | Nil | `log_error("Hello")`|
| generate_css_selector | generate Css Selector pattern for Xss payloads | String | `generate_css_selector("<img/src=x onerror=alert(1)")`
| html_parse | get the type of your payload in the response page | List of Location Enum | `html_parse("<h1 hackerman><h1>","hackerman") -- AttrName`  | 
| html_search | Search with CSS Selector in HTML | String | `html_search("<h1 hackerman>demo</h1>","h1")`
| change_urlquery | add your payload to all url parameters | Table (List) | `change_urlquery("http://google.com/?hello=1","hacker")` |
| set_urlvalue | Change custom parameter value in the url|  String | `set_urlvalue("http://google.com/?test=1","test","hacker")`|
| urljoin | Join Path to the url | String | `urljoin("http://google.com/","/search")` | 
| send_req | send Get http request to the url |  Table with ( url , status , body , errors ) | `send_req("https://google.com")` |

