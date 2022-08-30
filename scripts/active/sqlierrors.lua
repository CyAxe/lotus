-- Script Information
script_info = {}
script_info["name"] = "SQLIErrDetector"
script_info["methods"] = "GET"
script_info["type"] = "active_scan"
script_info["severity"] = "high"

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

function scan(url,payload)
    found = {}
    resp = send_req(url)
    for index_key,index_value in ipairs(sqli_errors) do
        match = is_match(index_value,resp.body:GetStrOrNil()) 
        if match == false then
            -- NOTHING
        else
            log_info(string.format("NOT SQLI FOUND:  %s | %s",resp.url:GetStrOrNil(),index_value))
            found[resp.url:GetStrOrNil()] = index_value
            break
        end
    end
    return found
end

function generate_payload(url,param)
    log_info(url)
    urls = {}
    for index_key, payload_value in ipairs(payloads) do
        local new_query = set_urlvalue(url,param,payload_value)
        log_debug(string.format("SQLIErrDetector > NEW QUERY: %s",new_query))
        urls[new_query] = payload_value
    end
    return urls
end
