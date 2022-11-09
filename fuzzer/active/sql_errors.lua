SQLI_ERRORS = read(string.format("%s/txt/sqli_errs.txt",SCRIPT_PATH))

PAYLOADS = {
    "'123",
    "''123",
    "`123",
    "\")123",
    "\"))123",
    "`)123",
    "`))123",
    "'))123",
    "')123\"123",
    "[]123",
    "\"\"123",
    "'\"123",
    "\"'123",
    "\123",
}

local function send_report(url,parameter,payload,matching_error)
    NewReport:setName("SQL Injection")
    NewReport:setDescription("https://owasp.org/www-community/attacks/SQL_Injection")
    NewReport:setRisk("high")
    NewReport:setUrl(url)
    NewReport:setParam(parameter)
    NewReport:setAttack(payload)
    NewReport:setEvidence(matching_error)
    print_report(NewReport)
end

function main(url) 
    local resp = http:send("GET",HttpMessage:getUrl())
    if resp.errors:GetErrorOrNil() then
        local log_msg = string.format("[SQLI_ERRORS] Connection Error: %s",new_url)
        log_error(log_msg)
        return
    end
    for param_index, param_name in pairs(HttpMessage:getParams()) do
        STOP_PARAM = false
        for payload_index, payload in pairs(PAYLOADS) do 
            local new_url = HttpMessage:setParam(param_name,payload)
            local resp = http:send("GET",new_url)
            local body = resp.body:GetStrOrNil()
            if STOP_PARAM == true then
                break
            end
            for sqlerror_match in SQLI_ERRORS:gmatch("[^\n]+") do
                    local match = is_match(sqlerror_match,body)
                    if ( match == false or match == nil) then
                            -- NOTHING
                    else
                        send_report(resp.url:GetStrOrNil(),param_name,payload,sqlerror_match)
                        Reports:addReport(NewReport)
                        STOP_PARAM = true
                        break
                    end
            end
        end
    end
end
