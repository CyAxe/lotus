REPORT = {}
VALID = false
STOP_AFTER_MATCH = true
THREADS = 15

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

function tablelength(T)
  local count = 0
  for _ in pairs(T) do count = count + 1 end
  return count
end


function main(current_payload,url)
    local n = tablelength(REPORT)
    local report = {}
    if n > 0 then
        print(n)
        print("YEAHOOO")
        VALID = false
        return report
     end
    http:set_proxy()
    local resp = http:send("GET",url,"")
    if resp.errors:GetErrorOrNil() then
        local log_msg = string.format("[SQLI] Connection Error: %s",new_url)
        log_error(log_msg)
        return REPORT
    end

    for err in SQLI_ERRORS:gmatch("[^\n]+") do
        local match = is_match(err,resp.body:GetStrOrNil()) 
        if ( match == false or match == nil) then
            -- NOTHING
        else
            report["url"] = url
            report["match"] = err
            report["payload"] = current_payload
            VALID = true
            println(string.format("SQLI ERROR: %s | %s |%s",current_payload,err,url))
            break
        end
    end
    table.insert(REPORT,report)
    return report
end

function payloads_gen(url)
    all_payloads = {}
    for index_key, payload_value in ipairs(PAYLOADS) do
        new_querys = change_urlquery(url,payload_value)
        for pay_index, pay_value in pairs(new_querys) do
            all_payloads[payload_value] = pay_value
        end
    end
    return all_payloads
end

