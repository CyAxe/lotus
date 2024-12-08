# 🌟 Lotus Lua API Documentation

Welcome to **Lotus Lua**, a powerful scripting platform for web security automation! This guide introduces you to key features and demonstrates how to write exceptional Lua scripts with the latest updates. Built on the blazing-fast combination of **LuaJIT** and **Rust**, Lotus ensures unmatched speed and efficiency for your scripting needs.

## 🚀 Quick Start

### 📝 Example Script

```lua
SCAN_TYPE = "URLS" -- Target a specific type of scan using its name

function main()
    println("Hello, Lotus!")
    local target = lotus.http:url()
    println("Scanning: " .. target)
end
```

Run your script with:

```bash
echo "http://example.com" | lotus scan my_script.lua
```

---

## 🔑 Key Features

### 🌐 HTTP Requests (`lotus.http:send`)

Effortlessly send HTTP requests:

- **GET Request**:
  ```lua
  local resp = lotus.http:send{url = "https://example.com"}
  println(resp.body)
  ```
- **POST Request**:
  ```lua
  local resp = lotus.http:send{
      method = "POST",
      url = "http://example.com/api",
      body = '{"key":"value"}',
      headers = {["Content-Type"] = "application/json"}
  }
  println(resp.status)
  ```

### ✍️ Encoding Utilities (`lotus.encode`)

Simplify data encoding and decoding:

- **Base64**:
  ```lua
  local encoded = lotus.encode.base64encode("hello")
  println(encoded) -- "aGVsbG8="
  local decoded = lotus.encode.base64decode(encoded)
  println(decoded) -- "hello"
  ```
- **URL Encoding**:
  ```lua
  local safe = lotus.encode.urlencode("Hello Lua")
  println(safe) -- "Hello%20Lua"
  println(lotus.encode.urldecode(safe)) -- "Hello Lua"
  ```

### 🛠️ Logging (`log_*`)

Streamline debugging with professional logs:

```lua
log_info("Scan started.")
log_debug("Debugging mode.")
log_warn("Warning: Potential issue.")
log_error("Critical error occurred.")
```

---

## ⚡ Advanced Features

### 🔍 Parameter Fuzzing

Automate parameter scanning with `ParamScan`:

```lua
function scan(param_name, payload)
    local new_url = HttpMessage:param_set(param_name, payload)
    return lotus.http:send{url = new_url}
end

function callback(data)
    if data.body:find("vulnerable") then
        Reports:add({url = data.url, payload = data.payload})
    end
end

ParamScan:start_scan()
ParamScan:add_scan("param_name", {"payload1", "payload2"}, scan, callback, 5)
```

### 📊 Report Your Findings

Efficiently log findings with the `Reports` utility:

```lua
Reports:add({
    url = "http://example.com",
    match = {"vulnerability found"}
})
```

---

## 🌟 Why Choose Lotus?

Lotus stands out by combining:

- **⚡ LuaJIT**: A Just-In-Time Compiler for Lua, ensuring high-performance scripting.
- **🦀 Rust**: A modern, safe, and fast systems programming language that powers the backend, delivering stability and speed.

This combination provides exceptional speed and reliability, empowering developers to build powerful automation scripts with ease.

---

## ❓ FAQ

**1. How can I add more functionality?**
Install libraries from [luarocks.org](https://luarocks.org/) or request features on [GitHub](https://github.com).

**2. Is Lotus free for commercial use?**
Yes, but changes to core code must be shared under GPLv2. For special licensing, you can email me at `knassar702@gmail.com`.

---

Feel free to enhance your scripts, experiment with features, and share your feedback! 🚀

