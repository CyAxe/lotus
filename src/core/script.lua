
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

function matcher(resp)
    for index_key,index_value in ipairs(sqli_errors) do
        match = is_match(index_value,resp.body:GetStrOrNil()) 
        if match == false then
            -- NOTHING
        else
            log_info(string.format(">> FOUND:  %s | %s",resp.url:GetStrOrNil(),index_value))
            return 1
        end
    end
end

function main(url)
    for index_key, payload_value in ipairs(payloads) do
        url_query = change_urlquery(url,payload_value)
        for param_key, full_url in next, url_query do
            resp = send_req(string.format("%s",full_url))
            log_info(string.format("SENDING REQUEST TO %s",full_url))
            if resp.errors:GetErrorOrNil() == nil then
                if matcher(resp) == 1 then
                    break
                end
            else
                log_info(string.format("%s", resp.errors:GetErrorOrNil()))
            end
        end
    end
    report = {}
    report["found"] = true
    return report
end
