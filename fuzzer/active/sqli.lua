REPORT = {}
VALID = false
STOP_AFTER_MATCH = true
THREADS = 2

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

function main(param,url)
    local resp = send_req(url)
    if resp.errors:GetErrorOrNil() then
        return REPORT
    end

    for err in SQLI_ERRORS:gmatch("[^\n]+") do
        local match = is_match(err,resp.body:GetStrOrNil()) 
        if ( match == false or match == nil) then
            -- NOTHING
        else
            REPORT["url"] = url
            REPORT["match"] = index_value
            REPORT["payload"] = current_payload
            VALID = true
            println(string.format("SQLI ERROR: %s",url))
            break
        end
    end
    return REPORT
end

function payloads_gen(url)
    all_payloads = {}
    if string.find(url,"?") then
        for index_key, payload_value in ipairs(PAYLOADS) do
            new_querys = change_urlquery(url,payload_value)
            for pay_index, pay_value in pairs(new_querys) do
                table.insert(all_payloads,pay_value)
            end
        end
    end
    return all_payloads
end

