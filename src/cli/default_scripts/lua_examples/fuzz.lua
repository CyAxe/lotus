--[[
This Lua script example is a fuzzing implementation for the Lotus project. The script conducts a GET parameter scan by initiating multi-threading on all URL parameters with LuaThreader. Then, each parameter is passed one by one to the scan_sqli function, which attempts to scan the target parameter by adding double quotes to its value.

Next, the script performs matching using regular expressions to check if any SQL errors appear in the response. If a match is found, the scan_sqli function calls the send_report function to create a new report and save it to the JSON output file (-o out.json).
--]]


SCAN_TYPE = 2
local function send_report(url,parameter,payload,matching_error)
    Reports:add {
        name = "SQL Injection",
        description = "https://owasp.org/www-community/attacks/SQL_Injection",
        risk = "high",
        url = url,
        parameter = parameter,
        attack = payload,
        evidence = matching_error
    }
end
--[[
Global Functions

HttpMessage Contains the Url with a few functions that help you to change the parameters value and joining path
```lua
HttpMessage:setParam(param_name, payload_value) -- Change custom url parameter value 
HttpMessage:urlJoin("/admin/") -- Joining url path
HttpMessage:setAllParams(payload_value) -- inject the payload to all url parameters (return table list)
```

]]--


function scan_sqli(param_name)
    for payload_index, payload in pairs(PAYLOADS) do 
        -- Adding the payload value into the target parameter
        local new_url = HttpMessage:setParam(param_name,payload)
        local resp = http:send{ url = new_url }  -- Sending a http request to the new url with GET Method
        local body = resp.body -- Get the response body as string
        local sqlerror_match = "SQL syntax.*?MySQL" -- The SQL Error Regex
        local status, match = pcall(function ()
            -- Matching with the response and the targeted regex
            -- we're using pcall here to avoid regex errors (and panic the code)
            return is_match(sqlerror_match, body)
            -- Use ResponseMatcher Lotus Functions if you need more Regex Options
        end)
        -- Check if there's No errors from this call
        -- Source: https://www.lua.org/pil/8.4.html
        if status ~= nil then
            if ( match == false or match == nil) then
                    -- Doesn't match
            else
                -- Matched, send the url as string and the target parameter and the matched regex to report it
                url = resp.url
                send_report(url, param_name,payload,sqlerror_match)
                break -- stop the current scanning to the thread
            end

        end
    end
end

function main(url) 
    -- Creating multi-threading task by using the url parameters iterator
    LuaThreader:run_scan(HttpMessage:Params(), scan_sqli, 30)
    -- ensure that your threading target function is GLOBAL not local function
    -- LuaThreader:run_scan(TARGET_TABLE, TARGET_FUNCTION, NUMBER_THREADS)
end
