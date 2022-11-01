<p align="center">
<img src="https://user-images.githubusercontent.com/45688522/187603703-5781b86b-9f5a-4658-9370-7083a3b5b6d5.png" width="470px">
</p>


:warning: We're Looking for maintainers
https://github.com/rusty-sec/lotus/issues/39

# lotus


:zap: Fast Web Security Scanner written in Rust based on Lua Scripts :waning_gibbous_moon: :crab: 


Currently this project is still under beta version, there are alot of features that are still under developing
it would be better if you make a contribute to this project to make it finish faster, you can check the project [issues page](https://github.com/rusty-sec/lotus/issues) for more, 
Don't forget to [Join Us on Discord](https://discord.gg/pxw57D4v)
### Usage
you can build it from source 
```bash
$ cargo install --git=https://github.com/rusty-sec/lotus/
```

or download the binary file from [the release page](https://github.com/rusty-sec/lotus/releases)

```bash
❯ echo "http://testphp.vulnweb.com/listproducts.php?cat=1" | lotus --scripts fuzzer/active --workers 30 --output test.json
🔥 RXSS: http://testphp.vulnweb.com/listproducts.php?cat=1%22%3E%3Cimg+src%3Dx+onerror%3Dalert%28%29%3E | "><img src=x onerror=alert()> | img[onerror="alert()"][src="x"]

❯ cat test.json | jq
[
  {
    "risk": "medium",
    "name": "reflected cross site scripting",
    "description": "https://owasp.org/www-community/attacks/xss/",
    "url": "http://testphp.vulnweb.com/listproducts.php?cat=1%22%3E%3Cimg+src%3Dx+onerror%3Dalert%28%29%3E",
    "param": "cat",
    "attack": "\"><img src=x onerror=alert()>",
    "evidence": "img[src=\"x\"][onerror=\"alert()\"]"
  }
]
[
  {
    "risk": "high",
    "name": "SQL Injection",
    "description": "https://owasp.org/www-community/attacks/SQL_Injection",
    "url": "http://testphp.vulnweb.com/listproducts.php?cat=1%27123",
    "param": "cat",
    "attack": "'123",
    "evidence": "check the manual that (corresponds to|fits) your MySQL server version"
  },
  {
    "risk": "high",
    "name": "SQL Injection",
    "description": "https://owasp.org/www-community/attacks/SQL_Injection",
    "url": "http://testphp.vulnweb.com/listproducts.php?cat=1%27%27123",
    "param": "cat",
    "attack": "''123",
    "evidence": "check the manual that (corresponds to|fits) your MySQL server version"
  },
```bash
Lotus 0.2-beta
Khaled Nassar <knassar702@gmail.com>
Fast Web Security Scanner written in Rust based on Lua Scripts

USAGE:
    lotus [OPTIONS] --workers <workers> --scripts <scripts> --output <output> [nolog]

ARGS:
    <nolog>    no logging

OPTIONS:
    -h, --help                               Print help information
    -l, --log <log>                          Save all lots to custom file
    -o, --output <output>                    Path of the JSON output fiel
    -s, --scripts <scripts>                  Path of scripts dir
    -t, --script-threads <script_threads>    Workers for lua scripts [default: 5]
    -V, --version                            Print version information
    -w, --workers <workers>                  Number of works of urls [default: 10]
```



### Lua API

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
    

#### Enum

To get the value from lua script you can call it with `value:GetEnumTypeOrNil`
- send_req 

```rust
pub enum RespType {
    NoErrors,
    Emtpy,
    Str(String),
    Int(i32),
    Error(String),
}
```

```lua
local resp = send_req("http://google.com")
if resp.errors:GetErrorOrNil() == nil then
  -- NO Connection ERRORS
  if string.find(resp.body:GetStrOrNil(),"google") then
    log_info("FOUND GOOGLE")
  end
end
  
```


- html_parse

```rust
pub enum Location {
    AttrValue(String),
    AttrName(String),
    TagName(String),
    Text(String),
    Comment(String),
}

```

```lua
local searcher = html_parse("<h1>Hello</h1>","Hello")
for index_key,index_value in ipairs(searcher) do
  if index_value:GetTextOrNil() then
    println(string.format("FOUND IT IN TEXT %s",index_value:GetTextOrNil()))
  end
end
```

