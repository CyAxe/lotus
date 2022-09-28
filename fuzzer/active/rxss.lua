REPORT = {}
VALID = false
STOP_AFTER_MATCH = true
THREADS = 1


PAYLOADS = read(string.format("%s/txt/xss.txt",SCRIPT_PATH))

function payloads_gen(url)
    all_payloads = {}
    for payload in PAYLOADS:gmatch("[^\n]+") do
        new_querys = change_urlquery(url,payload)
        for pay_index, pay_value in pairs(new_querys) do
            all_payloads[payload] = pay_value
        end
    end
    return all_payloads
end


function main(current_payload,new_url)
    local resp = send_req(new_url)
    if resp.errors:GetErrorOrNil() then
        return REPORT
    end
    local body = resp.body:GetStrOrNil()
    local css_pattern = generate_css_selector(current_payload)
    if string.len(css_pattern) > 0 then
        local searcher = html_search(body,css_pattern)
        if string.len(searcher) > 0 then
            println(string.format("RXSS: %s | %s | %s ",resp.url:GetStrOrNil(),"",css_pattern))
            REPORT["url"] = new_url
            REPORT["match"] = css_pattern
            REPORT["payload"] = current_payload
            VALID = true
        end
    end
    return REPORT
end
