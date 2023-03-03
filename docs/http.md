
### Starting Point
Make the main() function globally accessible, and try the Lotus utilities to write a great script but make sure to set your Script type first


| SCAN ID | INPUT TYPE                    |                                           Example |
| ---     | ---                           |                                               --- |
| 1       | HOSTS                         |                             `testphp.vulnweb.com` |
| 2       | FULL URL Including Parameters | `http://testphp.vulnweb.com/artists.php?artist=1` |
| 3       | Passing URL Paths only without Parameters | `http://testphp.vulnweb.com/artists.php?artist=1` |

```lua 
-- hacking_script.lua

SCAN_TYPE = 2

function main() 
    println("Hello World :D")
end
```
and then call it

```bash
$ echo "http://target.com" | lotus urls hacking_script.lua -o out.json 
Hello World :D
```

### DEV Enviroment

Use any editor you want but ensure that you've installed the Lua Server Plugin
and then open the lotus scripts repo you will be able to use automcomplete for lotus function
finished your script and want to publish it?
```bash
$ git checkout -b new_script
$ git add myscript.lua
$ git commit -m 'new script for X'
$ git push origin new_script
```


### Network

> at the moment, lotus 0.4-beta is only sending http requests via one http library that means you cannot send a requests by using Socket or DNS, we're planning to add this in the upcoming version


* [Changing the URL Query](#changing-the-url-query)
* [Sending HTTP Requests](#http-requests)
* [Chaning the Default Connection options](#change-the-request)
* [Handling Connection Errors](#handle-connection-errors)

#### Changing the URL Query
It is possible to use the HttpMessage Lua Class to get your target URL, with this class you are able to perform the following:
- Get the target URL
```lua
-- echo "http://target.com/?is_admin=true" | lotus urls script.lua -o out.json
local target_url = HttpMessage:Url()
-- http://target.com
```

- Getting all parameters in String
```lua
-- echo "http://target.com/?is_admin=true&year=2023" | lotus urls script.lua -o out.json
local params = HttpMessage:TxtParams()
-- "is_admin=true&year=2023"
```
- Get iterator with all url query
```lua
-- echo "http://target.com/?is_admin=true&year=2023" | lotus urls script.lua -o out.json
local iter_params = HttpMessage:Params()

for param_name, param_value in ipairs(iter_params) do 
    -- param_name: is_admin
    -- param_value: true
end
```
- Changing the value of custom Parameter
```lua
-- URL = https://target.com/users?name=Mike&age=20
local new_url = HttpMessage:setParam("age","23")
-- https://target.com/users?name=Mikehacker&age=2023
Changing the value of all parameters
-- URL = https://target.com/users?name=Mike&age=20
local new_params = HttpMessage:setAllParams("<h1>",true) -- true = remove the parameter value
for param_name,param_value in ipairs(new_params) do 
    -- param_name: name
    -- param_value: <h1>
    -- continue ..
end
```
- Join URL Path for root path
```lua
make sure to make the global variable SCAN_TYPE value to 3 to make lotus pass the full path instead of parameters to avoid dups inputs
-- URL = https://target.com/users?name=Mike&age=20
local new_url = HttpMessage:urlJoin("/admin/login?login=true")
-- URL = https://target.com/admin/login?login=true
Join URL Path for current path
-- make sure that your path doesn't starts with /
local new_url = pathjoin(HttpMessage:Path(),"admin/login.php")
-- http://target.com/index.php/admin.login.php
```

### HTTP Requests
Your lua script must call the HTTP lua class whose methods are assigned to the rust HTTP module in order to send HTTP requests
Send any method that you wish with a body and headers, but make sure that the headers are in Lua tables rather than strings
Sending normal GET request
Using the 'http:send()' function will permit you to send an HTTP request directly, but make sure you add the method and the URL first since these fields are required by the function Keep in mind that http:send takes the connection options from the user options. If you need to change the connection options for your script, you can visit #change-the-request.

```lua
local resp = http:send("GET", "https://google.com")
by adding this line you will call the https://google.com with GET method you will recive table with the response body/headers/url

local resp = http:send("GET", "https://google.com")
println(resp.body) -- use println function to print message above the progress bar
for header_name,header_value in ipairs(resp.headers) do 
    println(string.format("%s: %s",header_name, header_value))
end
```
- Sending POST Requests
```lua
local headers = {}
headers["X-API"] = "RANDOM_DATA"
headers["Content-Type"] = "application/json"
local resp = http:send("POST","http://target.com/api/users",'{"user_id":1}',headers)
```
### Change the request
You can change the default http connection options of your script
- Connection timeout
```lua
http:set_timeout(10) -- 10 secs
```
- limits of redirects
```lua
http:set_redirects(1) -- no redirects
http:set_redirects(1) -- only one redirect
```
- Custom Proxy
```lua
http:set_proxy("http://localhost:8080")
```
keep in mind this will only works in your script not in all scripts, so every time you call http:send function, the options that you changed will be called
### Handle Connection Errors
When using the "http:send" function, you might encounter a connections error because of the target response, so to ensure your script is not panicked, call the function within the protect function in the Lua language. This statement only returns a boolean value indicating whether the function has errors or not. For more information about pcall, please see the following link.
```lua
local func_status, resp = pcall(function () 
        return http:send("GET","http://0.0.0.0") -- request the localhost
        end)
if func_status == true then 
    -- True means no errors
    println("MAN WAKE UP I CAN ACCESS YOUR LOCAL NETWORK")
end
```
Also you can tell lotus about the error by adding a logging lines for it
```lua
if func_status == true then 
    -- True means no errors
    println("MAN WAKE UP I CAN ACCESS YOUR LOCAL NETWORK")
else 
    log_error(string.format("Connection Error: %s",func_status))
end
```

#### what if you want to check for custom error message ?
For example, if you have a Time-based Blind SQL Scanner, the only way to
determine whether a parameter is vulnerable is to set your Connection Timeout
to a value lower than the value for the SQL SLEEP Function Therefore, you must
verify whether the error was caused by a connection timeout or not
This can be accomplished by adding this function to your LUA script, and then sending the pcall error output to the function along with the error string message
```lua
function error_contains(error_obj, error_msg)
    -- ERR_STRING => Converting Error message from error type to string
    return str_contains(ERR_STRING(error_obj),error_msg)
end


function main() 
    local status, resp = pcall(function () 
        return http:send("GET","http://timeouthost")
    end)
    if status ~= true then 
        local timeout_err = error_contains(resp,"caused by: runtime error: timeout_error")
        if timeout_err == true then 
            println("TIMEOUT ERROR")
        end
    end
end
```
#### Connection ERROR Table

| Error        | Lua Code             |
| ---          | ----                 |
| Timeout      | `timeout_error`      |
| Connection   | `connection_error`   |
| Request Body | `request_body_error` |
| Decode       | `decode_error`       |
| External     | `external_error`     |







### Text Matching
while writing your own script, you need to ensure that you have been matched the right text to avoid False Postive
lotus gives you many easy ways for text matching/procssing

- match with regex
```lua
SCAN_TYPE = 2
function main()
	local resp = http:send("GET","http://testphp.vulnweb.com/artists.php?artist=1")
	local body = resp.body
	local searched = html_search(body,"h2[id=\"pageName\"]")
	println(searched)
	-- <h2 id="pageName">artist: r4w8173</h2>
end
```
generating CSS Selector Pattern for XSS Payloads
you can use this for the XSS CVES, to ensure that the payload is render in the page or not
```lua
XSS_PAYLOAD = "<img src=x onerror=alert(1)>"
function main()
	local search_pattern = generate_css_selector(XSS_PAYLOAD)
	println(search_pattern)
	-- img[onerror="alert(1)"][src="x"]
end
```
- match with Regex
```lua
function main()
	local matched = is_match("\\d\\d\\d","123")
	println(string.format("MATCHED: %s",matched))
	-- MATCHED: true
end
```
- check if the string includes data
```lua
str_contains("I use lua","use") -- true
```
- check if the string startswith
```lua
str_startswith("I use lua","I use") -- true
```
- text matching with and / or conditions
```lua
SCAN_TYPE = 2

function main()
	local match_one = {"test","Mike"}
	local match_all = {"Mike","true"}
	local BODY = '{"name":"Mike","is_admin":true}'
	-- match body with `or` conditions
	-- it means the function will returns true if one of the elements in the list matched with the body
	ResponseMatcher:match_body_once(BODY,match_one) -- true
	-- match body with `and` conditions
	-- it means the function will returns true if all of the elements in the list matched with the body
	ResponseMatcher:match_body(BODY,match_all) -- true
end
```



## Reporting

Lotus is giving you one simple way to report/save the output of your script, you have two types
General information
Vuln report
CVE report
General information
every time you run a script lotus would expect a list of findings in your report, it means you can include many finidings in the same report and the script as well so first you've to set the report information and after that call a global Lua Class called Reports

##### Vuln Report
```lua
local function send_report(url,parameter,payload,matching_txt)
    VulnReport:setName("Template Injection") -- vulnerability name
    VulnReport:setDescription("https://owasp.org/www-project-web-security-testing-guide/v41/4-Web_Application_Security_Testing/07-Input_Validation_Testing/18-Testing_for_Server_Side_Template_Injection") -- simple description
    VulnReport:setRisk("high") -- vulnerability risk
    VulnReport:setUrl(url) -- Vulnrable URL
    VulnReport:setParam(parameter) -- Vulnrable Parameter
    VulnReport:setAttack(payload) -- Used Payload
    VulnReport:setEvidence(matching_txt) -- Matched Text
    print_vuln_report(VulnReport) -- Print The Current Value of the report to the CLI
    Reports:addVulnReport(VulnReport) -- Save the current value of the Class to the script report list
end

function main() 
    -- SOME LOGIC
    send_report("http://hello.com/?name=%7B%7B2%2A2%7D%7D","name","%7B%7B2%2A2%7D%7D","4")
end
```

##### CVE Report
in CVE report you can add many Matchers like if you have a CVE that detect a vulnrablite based on the response header and body soo you have to include that in the report, to fix that you can use CveReport:addMatcher function to add whatever you want based on the Match ID

> The Match ID is allocating for custom part of the response

| Match ID | Match Type         |
| ---      | --                 |
| 1        | Full Response      |
| 2        | Response Headers   |
| 3        | Response Body      |
| 4        | Status Code        |
| 5        | General (anything) |

- Full Response
```lua
CveReport:addMatcher("<h1>Hi</h1>",1)
```
- Response Headers
```lua
CveReport:addMatcher("Content-Type: text/html",2)
```
- Response Body
```lua
CveReport:addMatcher("<h1>H1</h1>",3)
```
- Response Status
```lua
CveReport:addMatcher("301",4)
```
- General (Request or Response)
```lua
CveReport:addMatcher("IDK WHERE I FOUND THAT HONESTLY",5)
```
when you can the send_report function all matches that you added will be mentioned the in report, if you want to clear the matchers for example adding a new finding in the same script you can use CveReport:clearMatcher() to clear the matching list
```lua
local function send_report(url)
    CveReport:setName("CVE-2020-11450") -- CVE Name
    CveReport:setDescription("MicroStrategy Web 10.4 is susceptible to information disclosure. The JVM configuration, CPU architecture, installation folder, and other information are exposed through /MicroStrategyWS/happyaxis.jsp. An attacker can use this vulnerability to learn more about the application environment and thereby possibly obtain sensitive information, modify data, and/or execute unauthorized operations.") -- Description
    CveReport:setRisk("high") -- Risk
    CveReport:setUrl(url)  -- URL
    Reports:addCveReport(CveReport) -- Save The Current value of CveReport to report list
    print_cve_report(CveReport) -- Print CVE report to CLI
    CveReport:clearMatcher() -- Clear the matching List
end

function main() 
    -- Some Logic 
    CveReport:addMatcher('url: xss://"-alert(document.domain)',3)  -- 3 = response body
    CveReport:addMatcher('text/html',2)  -- 2 = response headers
    CveReport:addMatcher("200",4)  -- 4 = response status
    send_report("http://target.com/wp-content/plugins/embed-swagger/swagger-iframe.php?url=xss://%22-alert(document.domain)-%22")
end
```


### Logging
| Log Level | Lua Function |
| ---       | --           |
| INFO      | `log_info`   |
| DEBUG     | `log_debug`  |
| WARN      | `log_warn`   |
| ERROR     | `log_error`  |


```lua
local main()
    log_debug("Hello MOM :D")
end
```


```bash
$ echo "http://target.com"| lotus urls main.lua -o out.json --log log.txt
$ cat log.txt
[2023-02-28][14:40:09][lotus::cli::bar][INFO] URLS: 1
[2023-02-28][14:40:09][lotus::lua::parsing::files][DEBUG] READING "main.lua"
[2023-02-28][14:40:09][lotus][DEBUG] Running PATH scan 0
[2023-02-28][14:40:09][lotus::lua::parsing::files][DEBUG] READING "main.lua"
[2023-02-28][14:40:09][lotus][DEBUG] Running URL scan 1
[2023-02-28][14:40:09][lotus][DEBUG] Running main.lua script on http://target.com
[2023-02-28][14:40:09][lotus::lua::loader][DEBUG] Hello MOM :D
```



### Fuzzing

lotus is focusing to make the fuzzing or multi-threading process easy and simple by providing two class to help in common fuzzing cases


the first one is  for parameter scanning that doesn't means this the can be used for Param Scanner this but the idea is this class has been created for that reason

##### ParamScan
this class takes one string with List, for the target parameter to scan and the payloads list, after that the ParamScan class will send the target parameter with every item in the payloads list to the target function
> target function is just lua function you create to so simple thing like sending http requests and return the response  

after sending it to the target function it will take the output of this function and then send it to the callback function

> Callback function is list the target function but for parsing 


in you callback function parse the target function output and see if this able is valid to save it in the report or not 

> FUZZ_WORKERS is lua varaible the value of --fuzz-workers option
```lua
SCAN_TYPE = 2

local function send_report(url,parameter,payload,matching_error)
    VulnReport:setName("Template Injection")
    VulnReport:setDescription("https://owasp.org/www-project-web-security-testing-guide/v41/4-Web_Application_Security_Testing/07-Input_Validation_Testing/18-Testing_for_Server_Side_Template_Injection")
    VulnReport:setRisk("high")
    VulnReport:setUrl(url)
    VulnReport:setParam(parameter)
    VulnReport:setAttack(payload)
    VulnReport:setEvidence(matching_error)
    print_vuln_report(VulnReport)
end

SSTI_PAYLOADS = {
    "lot{{2*2}}us",
    "lot<%= 2*2 %>us"
}

function scan_ssti(param_name,payload)
    local new_url = HttpMessage:setParam(param_name,payload)
    local resp_status,resp = pcall(function ()
        return http:send("GET",new_url) -- Sending a http request to the new url with GET Method
    end)
        if resp_status == true then
            local out = {}
            local body = resp.body -- Get the response body as string
            out["body"] = body
            out["url"] = resp.url
            out["param_name"] = param_name
            out["payload"] = payload
            return out
        end
end

function ssti_callback(data)
    if data == nil then
        return -- avoid nil cases
    end
    url = data["url"]
    body = data["body"]
    payload = data["payload"]
    param_name = data["param_name"]
    local match_status, match = pcall(function () 
        -- Matching with the response and the targeted regex
        -- we're using pcall here to avoid regex errors (and panic the code)
        return str_contains(body, "lot4us")
    end)
    if match_status == true then
        if match == true then
            send_report(url,param_name,payload,"lot4us")
            Reports:addVulnReport(VulnReport)
        end
    end
end

function main()
    for _,param in ipairs(HttpMessage:Params()) do
        ParamScan:start_scan()
        ParamScan:add_scan(param,SSTI_PAYLOADS, scan_ssti,ssti_callback, FUZZ_WORKERS)
    end
end
```

Basically, we are doing a for loop on all url parameters in the code above and
then creating a scanning thread with the target parameter, the SSTI_PAYLOAD
List, scan_ssti as the target function and ssti_callback as the callback
function, and FUZZ_WORKERS is a lua variable that gets its value from the
`--fuzz-workers` parameter (you can replace it with real number of you want) 

As part of the ssti_scan function, we change the parameter value to the SSTI
payload, and then send an HTTP request to it, and return a list with the
following components: body, url, payload, parameter name. 

ParamScan will then take the output of this function and pass it to the function callback
(ssti_callback). in the call callback function first lines it checks if the
function parameter value is nil (Null) or not because doing any match You may
set this option to prevent ParamScan from sending Nil to the call_back
functions

```lua
ParamScan:accept_nil(false) -- Dont pass any nil values
ParamScan:is_accept_nil() -- check if ParamScan is passing nil values or not
```
If you are scanning parameters, you do not need to call any of these functions since the default option is not to pass any null values to them
From anywhere in your script, you may call the ParamScan:stop_scan() function to stop the scanner and clear all futures
You can disable this option by using the ParamScan:start_scan() function
and if you want to check first if ParamScan is stopped or not you can use ParamScan:is_stop()

#### LuaThreader
this a simple class to do multi-threading, it only takes iterator and function to run 
```lua
SCAN_TYPE = 2

PAYLOADS = {
    "hello",
    'world'
}
function SCANNER(data)
    -- DO YOUR SCANNING LOGIC
end

function main()
    LuaThreader:run_scan(PAYLOADS,SCANNER,10) -- 10 = Number of workers
    -- LuaThreader:stop_scan() = stop the scan and dont accept any new futures
    -- LuaThreader:is_stop() = Check if LuaThreader is stopped or not
end
```
The LuaThreader class will open two threads in this example, one for the hello word and one for the world word
It is really as simple as that 



### Reading Files

- Reading files
```lua
local status, file = pcall(function()
    return readfile("/etc/passwd") 
end)
if status == true then 
    println(file)
end
```
- Path Join
```lua
pathjoin("/etc/","passwd") -- /etc/passwd
```
- Path Join in the script directory 
```lua
-- script dir /home/docker/scripts/main.lua
JOIN_SCRIPT_DIR("payloads/sqli.txt")
-- /home/docker/scripts/payloads/sqli.txt
```
- Convert files to iterators by new lines
```lua
local status, lines = pcall(function()
    return readfile("/etc/passwd") 
end)
if status == true then 
    for word in line:gmatch("%w+") do 
        --
    end 
end
```
you can see the offical Lua IO Library for more informations  



### Encoding 

- Base64
```lua
base64encode("hello") -- aGVsbG8=
base64decode("aGVsbG8=") -- hello
```



### FAQ

##### Comercial Use 
Thank you first for using lotus commercially
However, you should keep in mind that the Lotus Project is licensed under the GPLv2 license, which allows commercial use of the project, however it requires you to open a PR or inform the Lotus Project Team if you made any changes to the core code
Lotus is doing this because we want to ensure that everyone has access to all of its features
It does not mean that your lua scripts should be shared with others. We actually use BSD licenses for lua scripts, which allow you to hide your scripts according to your preferences

Would you like to discuss with the team the possibility of releasing Lotus in other license for you?
just send an email to knassar702@gmail.com
Feel free to send the same email if you need assistance with how to use Lotus effectively for your business
It would be great if you could join a meeting with the Lotus team and discuss this in more detail:)



##### I can't find the function that I need
you can download any library from https://luarocks.org/ and then import it in your script 
Or open an issue on our Github repository for the functionality you are missing
