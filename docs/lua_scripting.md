
## Overview

Your Rust application integrates Lua scripting capabilities using the `mlua` crate. To enhance Lua's functionality, you've extended the Lua runtime with various utility functions, including logging, string matching, threading, and encoding/decoding operations. These utilities are encapsulated within the `UtilsEXT` and `EncodeEXT` traits and are accessible in Lua under the `lotus` table.

---

## Table of Contents

1. [UtilsEXT Trait](#utilsext-trait)
   - [add_printfunc](#add_printfunc)
     - [`lotus.log_info(msg)`](#lotuslog_infomsg)
     - [`lotus.log_warn(msg)`](#lotuslog_warnmsg)
     - [`lotus.log_debug(msg)`](#lotuslog_debugmsg)
     - [`lotus.log_error(msg)`](#lotuslog_errormsg)
     - [`lotus.join_script_dir(new_path)`](#lotusjoin_script_dirnew_path)
     - [`lotus.println(msg)`](#lotusprintlnmsg)
   - [add_matchingfunc](#add_matchingfunc)
     - [`lotus.matcher`](#lotusmatcher)
     - [`lotus.str_startswith(str_one, str_two)`](#lotusstr_startswithstr_one-str_two)
     - [`lotus.str_contains(str_one, str_two)`](#lotusstr_containsstr_one-str_two)
     - [`lotus.str_endswith(str_one, str_two)`](#lotusstr_endswithstr_one-str_two)
   - [add_threadsfunc](#add_threadsfunc)
     - [`lotus.ParamScan`](#lotusparamscan)
     - [`lotus.LuaThreader`](#lotusluathreader)
     - [`lotus.thread_log(msg)`](#lotusthread_logmsg)
2. [EncodeEXT Trait](#encodeext-trait)
   - [add_encode_funcs](#add_encode_funcs)
     - [`lotus.encode.base64encode(input)`](#lotusencodebase64encodeinput)
     - [`lotus.encode.base64decode(input)`](#lotusencodebase64decodeinput)
     - [`lotus.encode.urlencode(input)`](#lotusencodeurlencodeinput)
     - [`lotus.encode.urldecode(input)`](#lotusencodeurldecodeinput)
     - [`lotus.encode.htmlencode(input)`](#lotusencodehtmlencodeinput)
     - [`lotus.encode.htmldecode(input)`](#lotusencodehtmldecodeinput)
3. [Usage Examples](#usage-examples)
   - [Logging Example](#logging-example)
   - [String Matching Example](#string-matching-example)
   - [Threading Example](#threading-example)
   - [Encoding/Decoding Example](#encodingdecoding-example)

---

## UtilsEXT Trait

The `UtilsEXT` trait provides utility functions for logging, string matching, and threading. These functions are integrated into Lua under the `lotus` table, allowing Lua scripts to perform various operations seamlessly.

### `add_printfunc`

**Purpose:** Adds logging and printing functions to the Lua runtime.

#### `lotus.log_info(msg)`

**Description:** Logs an informational message.

**Parameters:**
- `msg` (`String`): The message to log.

**Usage:**
```lua
lotus.log_info("This is an informational message.")
```

#### `lotus.log_warn(msg)`

**Description:** Logs a warning message.

**Parameters:**
- `msg` (`String`): The warning message to log.

**Usage:**
```lua
lotus.log_warn("This is a warning message.")
```

#### `lotus.log_debug(msg)`

**Description:** Logs a debug message.

**Parameters:**
- `msg` (`String`): The debug message to log.

**Usage:**
```lua
lotus.log_debug("Debugging application flow.")
```

#### `lotus.log_error(msg)`

**Description:** Logs an error message.

**Parameters:**
- `msg` (`String`): The error message to log.

**Usage:**
```lua
lotus.log_error("An error has occurred.")
```

#### `lotus.join_script_dir(new_path)`

**Description:** Joins a new path segment to the directory of the current script.

**Parameters:**
- `new_path` (`String`): The new path segment to append.

**Returns:**
- `String`: The combined path.

**Usage:**
```lua
local full_path = lotus.join_script_dir("subfolder/script.lua")
println("Full Path: " .. full_path)
```

#### `lotus.println(msg)`

**Description:** Prints a message using the global progress bar.

**Parameters:**
- `msg` (`String`): The message to print.

**Usage:**
```lua
lotus.println("Operation completed successfully.")
```

### `add_matchingfunc`

**Purpose:** Adds string matching utilities to the Lua runtime.

#### `lotus.matcher`

**Description:** A `ResponseMatcher` object configured with specific matching rules.

**Usage:**
```lua
if lotus.matcher:contains("Hello, World!", "World") then
    lotus.log_info("Match found!")
end
```

#### `lotus.str_startswith(str_one, str_two)`

**Description:** Checks if `str_one` starts with `str_two`.

**Parameters:**
- `str_one` (`String`): The string to check.
- `str_two` (`String`): The prefix string.

**Returns:**
- `Boolean`: `true` if `str_one` starts with `str_two`, else `false`.

**Usage:**
```lua
local result = lotus.str_startswith("Hello, World!", "Hello")
if result then
    lotus.log_info("String starts with 'Hello'")
end
```

#### `lotus.str_contains(str_one, str_two)`

**Description:** Checks if `str_one` contains `str_two`.

**Parameters:**
- `str_one` (`String`): The string to search within.
- `str_two` (`String`): The substring to search for.

**Returns:**
- `Boolean`: `true` if `str_one` contains `str_two`, else `false`.

**Usage:**
```lua
if lotus.str_contains("Hello, World!", "World") then
    lotus.log_info("String contains 'World'")
end
```

#### `lotus.str_endswith(str_one, str_two)`

**Description:** Checks if `str_one` ends with `str_two`.

**Parameters:**
- `str_one` (`String`): The string to check.
- `str_two` (`String`): The suffix string.

**Returns:**
- `Boolean`: `true` if `str_one` ends with `str_two`, else `false`.

**Usage:**
```lua
if lotus.str_endswith("Hello, World!", "World!") then
    lotus.log_info("String ends with 'World!'")
end
```

### `add_threadsfunc`

**Purpose:** Adds threading and concurrency-related utilities to the Lua runtime.

#### `lotus.ParamScan`

**Description:** A `ParamScan` object for handling thread parameters.

**Usage:**
```lua
-- Example usage of ParamScan
if lotus.ParamScan.finds.lock().unwrap() then
    lotus.log_info("Parameter scan found a match.")
end
```

#### `lotus.LuaThreader`

**Description:** A `LuaThreader` object for managing threads.

**Usage:**
```lua
-- Example usage of LuaThreader
local threader = lotus.LuaThreader
-- Implement thread management logic as needed
```

#### `lotus.thread_log(msg)`

**Description:** Logs messages in a thread-safe manner.

**Parameters:**
- `msg` (`String`): The message to log.

**Usage:**
```lua
lotus.thread_log("Thread-safe operation in progress.")
```

---

## EncodeEXT Trait

The `EncodeEXT` trait provides encoding and decoding functions, enabling Lua scripts to perform various data transformations. These functions are accessible under the `lotus.encode` table.

### `add_encode_funcs`

**Purpose:** Adds encoding and decoding functions to the Lua runtime.

#### `lotus.encode.base64encode(input)`

**Description:** Encodes a string into Base64.

**Parameters:**
- `input` (`String`): The string to encode.

**Returns:**
- `String`: The Base64-encoded string.

**Usage:**
```lua
local encoded = lotus.encode.base64encode("Hello, World!")
println("Base64 Encoded: " .. encoded)
```

#### `lotus.encode.base64decode(input)`

**Description:** Decodes a Base64-encoded string.

**Parameters:**
- `input` (`String`): The Base64 string to decode.

**Returns:**
- `String`: The decoded string.

**Usage:**
```lua
local decoded = lotus.encode.base64decode(encoded)
println("Base64 Decoded: " .. decoded)
```

#### `lotus.encode.urlencode(input)`

**Description:** Encodes a string for safe inclusion in a URL.

**Parameters:**
- `input` (`String`): The string to encode.

**Returns:**
- `String`: The URL-encoded string.

**Usage:**
```lua
local url_encoded = lotus.encode.urlencode("https://example.com/?key=value")
println("URL Encoded: " .. url_encoded)
```

#### `lotus.encode.urldecode(input)`

**Description:** Decodes a URL-encoded string.

**Parameters:**
- `input` (`String`): The URL-encoded string to decode.

**Returns:**
- `String`: The decoded string.

**Usage:**
```lua
local url_decoded = lotus.encode.urldecode(url_encoded)
println("URL Decoded: " .. url_decoded)
```

#### `lotus.encode.htmlencode(input)`

**Description:** Escapes HTML special characters in a string.

**Parameters:**
- `input` (`String`): The string to escape.

**Returns:**
- `String`: The HTML-escaped string.

**Usage:**
```lua
local html_encoded = lotus.encode.htmlencode("<div>Example</div>")
println("HTML Encoded: " .. html_encoded)
```

#### `lotus.encode.htmldecode(input)`

**Description:** Unescapes HTML special characters in a string.

**Parameters:**
- `input` (`String`): The HTML-escaped string to unescape.

**Returns:**
- `String`: The unescaped string.

**Usage:**
```lua
local html_decoded = lotus.encode.htmldecode(html_encoded)
println("HTML Decoded: " .. html_decoded)
```

---

## Usage Examples

Below are comprehensive examples demonstrating how to utilize the integrated utility and encoding functions within Lua scripts.

### Logging Example

```lua
-- Logging various levels of messages
lotus.log_info("Application started successfully.")
lotus.log_debug("Debugging initialization parameters.")
lotus.log_warn("Low disk space detected.")
lotus.log_error("Failed to connect to the database.")
```

**Expected Output:**
```
[INFO] Application started successfully.
[DEBUG] Debugging initialization parameters.
[WARN] Low disk space detected.
[ERROR] Failed to connect to the database.
```

### String Matching Example

```lua
-- Using string matching utilities
local text = "The quick brown fox jumps over the lazy dog."

if lotus.str_startswith(text, "The quick") then
    lotus.log_info("The text starts with 'The quick'.")
end

if lotus.str_contains(text, "brown fox") then
    lotus.log_info("The text contains 'brown fox'.")
end

if lotus.str_endswith(text, "lazy dog.") then
    lotus.log_info("The text ends with 'lazy dog.'.")
end
```

**Expected Output:**
```
[INFO] The text starts with 'The quick'.
[INFO] The text contains 'brown fox'.
[INFO] The text ends with 'lazy dog.'.
```

### Threading Example

```lua
-- Using threading utilities
-- Initialize LuaThreader
local threader = lotus.LuaThreader

-- Start a thread (hypothetical usage)
-- This is a placeholder; actual threading logic depends on LuaThreader implementation
-- For demonstration purposes, we'll just log a message
lotus.thread_log("Thread-safe operation initiated.")

-- Using ParamScan
if lotus.ParamScan.finds.lock().unwrap() then
    lotus.log_info("Parameter scan detected a match.")
else
    lotus.log_info("Parameter scan did not find a match.")
end
```

**Expected Output:**
```
[INFO] Thread-safe operation initiated.
[INFO] Parameter scan detected a match.
```

### Encoding/Decoding Example

```lua
-- Encoding and decoding examples

-- Base64
local original = "Hello, World!"
local base64_encoded = lotus.encode.base64encode(original)
local base64_decoded = lotus.encode.base64decode(base64_encoded)
println("Base64 Encoded: " .. base64_encoded)
println("Base64 Decoded: " .. base64_decoded)

-- URL
local url = "https://example.com/?search=Lua+Integration"
local url_encoded = lotus.encode.urlencode(url)
local url_decoded = lotus.encode.urldecode(url_encoded)
println("URL Encoded: " .. url_encoded)
println("URL Decoded: " .. url_decoded)

-- HTML
local html = "<script>alert('XSS');</script>"
local html_encoded = lotus.encode.htmlencode(html)
local html_decoded = lotus.encode.htmldecode(html_encoded)
println("HTML Encoded: " .. html_encoded)
println("HTML Decoded: " .. html_decoded)
```

**Expected Output:**
```
Base64 Encoded: SGVsbG8sIFdvcmxkIQ==
Base64 Decoded: Hello, World!
URL Encoded: https%3A%2F%2Fexample.com%2F%3Fsearch%3DLua+Integration
URL Decoded: https://example.com/?search=Lua+Integration
HTML Encoded: &lt;script&gt;alert(&#x27;XSS&#x27;);&lt;/script&gt;
HTML Decoded: <script>alert('XSS');</script>
```

---

## Summary

This documentation provides a detailed overview of the utility and encoding functions integrated into your Lua runtime via the `UtilsEXT` and `EncodeEXT` traits. By leveraging these functions, Lua scripts can perform advanced operations such as logging, string matching, threading, and data encoding/decoding with ease and efficiency.

**Key Highlights:**

- **Logging Utilities:** Simplify logging at various levels (`info`, `warn`, `debug`, `error`) directly from Lua scripts.
- **String Matching:** Enhance string processing with functions to check prefixes, suffixes, and substring containment.
- **Threading Support:** Manage threading operations and ensure thread-safe logging.
- **Encoding/Decoding:** Facilitate data transformation with Base64, URL, and HTML encoding and decoding functions.

**Best Practices:**

- **Error Handling:** Always handle potential errors, especially when dealing with decoding functions that may receive invalid inputs.
- **Thread Safety:** Utilize thread-safe logging functions (`thread_log`) when performing operations in multi-threaded contexts.
- **Path Management:** Use `join_script_dir` to dynamically construct file paths relative to your Lua scripts, enhancing portability and flexibility.

By following this documentation, you can effectively utilize the integrated functions to build robust and maintainable Lua scripts within your Rust application.
