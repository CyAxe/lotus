<p align="center">
<img src="https://user-images.githubusercontent.com/45688522/187603703-5781b86b-9f5a-4658-9370-7083a3b5b6d5.png" width="470px">
</p>


# lottas


:zap: Fast Web Security Scanner written in Rust based on Lua Scripts :waning_gibbous_moon: :crab: 


Currently this project is still under beta version, there are alot of features that are still under developing
it would be better if you make a contribute to this project to make it finish faster, you can check the project [issues page](https://github.com/rusty-sec/lotus/issues) for more 

### Usage
you can build it from source 
```bash
$ cargo install --git=https://github.com/rusty-sec/lotus/
```

or download the binary file from the release page

```bash
$ lotus --output test.json --scripts scripts/
$ cat ~/lotus.log # logging file
$ cat test.json | jq
{
  "payload": "",
  "match_payload": "/secured/phpinfo.php",
  "url": "http://testphp.vulnweb.com/secured/phpinfo.php"
}
{
  "payload": "'123",
  "match_payload": "SQL syntax.*?MySQL",
  "url": "http://testphp.vulnweb.com/listproducts.php?cat=1%27123"
}
```


```bash
lotus 0.1.0
Fast Web Security Scanner written in Rust based on Lua Scripts

USAGE:
    lotus [OPTIONS] --output <json-output> --scripts <scripts>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <json-output>    Path of output JSON file
    -s, --scripts <scripts>       Path of Scripts dir
    -w, --workers <threads>       number of workers [default: 30]
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
if resp.errrors:GetStrOrNil() == nil then
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

