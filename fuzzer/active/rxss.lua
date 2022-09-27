REPORT = {}
VALID = false
STOP_AFTER_MATCH = true
THREADS = 1


PAYLOAD = '"><img src=x onerror=alert()>'
DATA = read(string.format("%s/txt/xss.txt",SCRIPT_PATH))
for s in DATA:gmatch("[^\n]+") do
    print(string.format("> %s",s))
end

function payloads_gen(url)
    new_querys = change_urlquery(url,PAYLOAD)
    return new_querys
end


function main(param,new_url)
    local resp = send_req(new_url)
    if resp.errors:GetErrorOrNil() then
        return REPORT
    end
    local body = resp.body:GetStrOrNil()
    local css_pattern = generate_css_selector(PAYLOAD)
    if string.len(css_pattern) > 0 then
        local searcher = html_search(body,css_pattern)
        if string.len(searcher) > 0 then
            println(string.format("RXSS: %s | %s | %s ",resp.url:GetStrOrNil(),PAYLOAD,css_pattern))
            REPORT["url"] = new_url
            REPORT["match"] = css_pattern
            REPORT["payload"] = PAYLOAD
            VALID = true
        end
    end
    return REPORT
end
