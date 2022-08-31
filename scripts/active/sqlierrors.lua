-- Script Information
script_info = {}
script_info["name"] = "SQLIErrDetector"
script_info["methods"] = "GET"
script_info["type"] = "active_scan"
script_info["severity"] = "high"
found = {}

sqli_errors = {
    'SQL syntax.*?MySQL',
    'Warning.*?\\Wmysqli?_', 
    'MySQLSyntaxErrorException',
    'valid MySQL result', 
    'check the manual that (corresponds to|fits) your MySQL server version', "Unknown column '[^ ]+' in 'field list'"
}

payloads = {
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

function scan(url,current_payload)
    resp = send_req(url)
    if resp.url:GetStrOrNil() == nil then
        return 0
    end

    for index_key,index_value in ipairs(sqli_errors) do
        match = is_match(index_value,resp.body:GetStrOrNil()) 
        if match == false then
            -- NOTHING
        else
            println(string.format("SQLI FOUND:  %s | %s",resp.url:GetStrOrNil(),index_value))
            found["url"] = resp.url:GetStrOrNil()
            found["match"] = index_value
            found["valid"] = true
            found["payload"] = current_payload
            return 1
        end
    end
    return 0
end

function main(url)
    stop = 0
    if string.find(url,"?") then
        for index_key, payload_value in ipairs(payloads) do
            new_querys = change_urlquery(url,payload_value)
            for url_index, new_url in pairs(new_querys) do 
                local out = scan(new_url, payload_value)
                if out == 1 then 
                    stop = 1
                    break
                end
            end
            if stop == 1 then
                break
            end
        end
    end
    return found
end
