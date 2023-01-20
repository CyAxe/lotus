local function send_report(url,parameter,payload,matching_error)
    VulnReport:setName("SQL Injection")
    VulnReport:setDescription("https://owasp.org/www-community/attacks/SQL_Injection")
    VulnReport:setRisk("high")
    VulnReport:setUrl(url)
    VulnReport:setParam(parameter)
    VulnReport:setAttack(payload)
    VulnReport:setEvidence(matching_error)
    print_report(VulnReport)
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
        local resp = http:send("GET",new_url) -- Sending a http request to the new url with GET Method
        local body = resp.body:GetStrOrNil() -- Get the response body as string
        local sqlerror_match = "SQL syntax.*?MySQL" -- The SQL Error Regex
        local status, match = pcall(function () 
            -- Matching with the response and the targeted regex
            -- we're using pcall here to avoid regex errors (and panic the code)
            return is_match(sqlerror_match,body)
        end)
        -- No errors from this call
        -- https://www.lua.org/pil/8.4.html
        if status ~= nil then 
            if ( match == false or match == nil) then
                    -- Doesn't match
            else
                -- Matched, send the url as string and the target parameter and the matched regex to report it
                send_report(resp.url:GetStrOrNil(),param_name,payload,sqlerror_match)
                Reports:addVulnReport(VulnReport)
                break -- stop the current scanning to the thread
            end

        end
    end
end

function main(url) 
    -- Creating multi-threading task by using the url parameters
    -- each thread will take the full url and you will find the target parameter in the function parameter value
    LuaThreader:run_scan(HttpMessage:getParams(), scan_sqli, 30)
    -- LuaThreader:run_scan(TARGET_TABLE, TARGET_FUNCTION, NUMBER_THREADS)
end
